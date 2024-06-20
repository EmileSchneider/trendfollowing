use crate::common::{Kline, Perpetual};
use chrono::Local;
use reqwest;
async fn get_klines(symbol: &str, limit: i32, end_time: i64) -> Result<Vec<Kline>, reqwest::Error> {
    let url = format!("https://fapi.binance.com/fapi/v1/klines?symbol={symbol}&interval=1d&limit={limit}&endTime={end_time}");
    println!("{}", url);
    let res = reqwest::get(url).await?;

    let content = &res.text().await;

    match content {
        Ok(content) => {
            let parsed: serde_json::Value =
                serde_json::from_str(content).expect("Could not parse the json");

            match parsed {
                serde_json::Value::Array(parsed) => {
                    let klines: Vec<Kline> = parsed
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
                        .collect();

                    return Ok(klines);
                }
                _ => Ok(vec![]),
            }
        }
        Err(_) => Ok(vec![]),
    }
}

async fn get_active_perps() -> Result<Vec<Perpetual>, reqwest::Error> {
    let res = reqwest::get("https://fapi.binance.com/fapi/v1/exchangeInfo").await?;
    let content = &res.text().await.expect("");
    let parsed: serde_json::Value =
        serde_json::from_str(content).expect("Could not parse the json");

    match parsed {
        serde_json::Value::Object(obj) => {
            let symbols = obj
                .get("symbols")
                .expect("Parsed object did not contain \"symbols\"!");
            match symbols {
                serde_json::Value::Array(symbols) => {
                    let active_perps = symbols
                        .into_iter()
                        .map(|sym| {
                            if let (
                                Some(serde_json::Value::String(symbol)),
                                Some(serde_json::Value::String(status)),
                                Some(serde_json::Value::String(contract_type)),
                            ) = (
                                sym.get("symbol"),
                                sym.get("status"),
                                sym.get("contractType"),
                            ) {
                                if contract_type == "PERPETUAL" && status == "TRADING" {
                                    Perpetual {
                                        symbol: symbol.to_string(),
                                    }
                                } else {
                                    Perpetual {
                                        symbol: "".to_string(),
                                    }
                                }
                            } else {
                                Perpetual {
                                    symbol: "".to_string(),
                                }
                            }
                        })
                        .filter(|perp| perp.symbol != "")
                        .collect();

                    return Ok(active_perps);
                }
                _ => {
                    eprintln!("Symbols wasn't an Array!");
                    return Ok(vec![]);
                }
            }
        }
        _ => Ok(vec![]),
    }
}

async fn scrape_all_klines(symbol: &str) -> Vec<Kline> {
    let mut all_klines: Vec<Kline> = vec![];
    let mut running: bool = true;
    let mut start_time: i64 = Local::now().timestamp() * 1000;

    while running {
        let res = get_klines(symbol, 500, start_time).await;
        match res {
            Ok(res) => {
                if res.first().expect("Res has not first").opentime == start_time {
                    running = false;
                } else {
                    all_klines.append(&mut res.clone().split_last().unwrap().1.to_vec());
                    all_klines.sort_by(|a, b| a.opentime.partial_cmp(&b.opentime).unwrap());
                    println!(
                        "FIRST: {}\tLAST:{}\tSTARTTIME: {}",
                        res.first().expect("").opentime,
                        res.last().expect("").opentime,
                        start_time
                    );
                    start_time = res[0].opentime;
                }
            }
            Err(_) => {
                eprintln!("Something went wrong!");
                running = false;
            }
        }
    }

    all_klines
}
