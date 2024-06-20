use crate::common::Kline;
use chrono::prelude::*;
use dotenv::dotenv;
use reqwest;
use sqlx::postgres::PgPoolOptions;

fn _update_data() {}

pub mod common;
pub mod database;

async fn get_klines(symbol: &str, last_entry: i64) -> Result<Vec<Kline>, reqwest::Error> {
    let url = format!("https://fapi.binance.com/fapi/v1/klines?symbol={symbol}&interval=1d&startTime={last_entry}");
    println!("{}", url);

    let res = reqwest::get(url).await?;

    let klines: Vec<Kline> = match &res.text().await {
        Ok(arg) => {
            let parsed: serde_json::Value = serde_json::from_str(arg).unwrap();
            if let serde_json::Value::Array(parsed) = parsed {
                parsed
                    .into_iter()
                    .map(|k| {
                        if let (
                            Some(serde_json::Value::Number(opentime)),
                            Some(serde_json::Value::String(open)),
                            Some(serde_json::Value::String(high)),
                            Some(serde_json::Value::String(low)),
                            Some(serde_json::Value::String(close)),
                            Some(serde_json::Value::String(volume)),
                        ) = (k.get(0), k.get(1), k.get(2), k.get(3), k.get(4), k.get(5))
                        {
                            Some(Kline {
                                opentime: opentime
                                    .as_i64()
                                    .expect("Fist value \"opentime\" needs to be an i64 "),
                                open: open.to_string(),
                                high: high.to_string(),
                                low: low.to_string(),
                                close: close.to_string(),
                                volume: volume.to_string(),
                            })
                        } else {
                            None
                        }
                    })
                    .filter_map(|v| v)
                    .collect()
            } else {
                println!("Parsing error:");
                std::process::exit(-1);
            }
        }
        Err(e) => {
            eprintln!("ERROR:{}", e);
            std::process::exit(-1);
        }
    };

    Ok(klines)
}
#[tokio::main]
async fn main() {
    println!("{:?}", Utc::now());
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("ERROR: Environment variable \"DATABASE_URL\" was not found!");

    let res = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await;

    let pool = match res {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(-1);
        }
    };

    let last_entries_result = sqlx::query!(
        "WITH LatestEntries AS (
    SELECT 
        coin_id, 
        MAX(opentime) AS MaxOpenTime
    FROM 
        daily_ohlcv
    GROUP BY 
        coin_id
)
SELECT 
    d.opentime,
    pf.symbol,
    d.coin_id
FROM 
    daily_ohlcv d
INNER JOIN 
    LatestEntries le ON d.coin_id = le.coin_id AND d.opentime = le.MaxOpenTime
INNER JOIN 
	perpetual_futures pf on d.coin_id = pf.id;",
    )
    .fetch_all(&pool)
    .await;

    let last_entries = match last_entries_result {
        Ok(val) => val,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(-1);
        }
    };
    for entry in last_entries {
        println!("{}\t{}", entry.opentime, entry.symbol);
        match get_klines(&entry.symbol, entry.opentime).await {
            Ok(klines) => {
                let klines = klines.split_last().unwrap().1.to_vec();
                for i in 1..(klines.len() - 1) {
                    let k = &klines[i];
                    println!("{:?}", k);

                    let insert_result = sqlx::query!("INSERT INTO daily_ohlcv(coin_id, opentime, openprice, highprice, lowprice, closeprice, volume) VALUES ($1, $2, $3, $4, $5, $6, $7)",
						     entry.coin_id,
						     k.opentime,
						     k.open,
						     k.high,
						     k.low,
						     k.close,
						     k.volume
		    ).execute(&pool).await;

                    if insert_result.is_err() {
                        eprintln!(
                            "Something went wrong inserting {:?} into {:?}",
                            k, entry.coin_id
                        );
                    }
                }
            }
            Err(e) => eprintln!("ERROR: {}", e),
        }
    }
}
