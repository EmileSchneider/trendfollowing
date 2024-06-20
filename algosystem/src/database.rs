use sqlx::postgres::{PgPool, PgPoolOptions};
use std::process::exit;

use crate::common::{Kline, Perpetual};

pub struct Db {
    pool: PgPool,
}

impl Db {
    async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let res = PgPoolOptions::new().max_connections(5).connect(url).await?;
        Ok(Db { pool: res })
    }
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
    pub async fn insert_perpetual_futures(
        &self,
        perpetual_futures: Vec<Perpetual>,
    ) -> Result<(), sqlx::Error> {
        for perp in perpetual_futures {
            let _res = sqlx::query!(
                "INSERT INTO perpetual_futures(symbol) VALUES ($1)",
                perp.symbol
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn get_all_perpetual_futures(&self) -> Result<Vec<Perpetual>, sqlx::Error> {
        let rows = sqlx::query!("SELECT symbol FROM perpetual_futures;")
            .fetch_all(&self.pool)
            .await?;

        let perps: Vec<Perpetual> = rows
            .into_iter()
            .map(|r| Perpetual { symbol: r.symbol })
            .collect();

        Ok(perps)
    }

    pub async fn insert_daily_klines(
        &self,
        symbol: &str,
        klines: Vec<Kline>,
    ) -> Result<(), sqlx::Error> {
        let symbol_id = sqlx::query!(
            "SELECT id FROM perpetual_futures WHERE symbol = ( $1 )",
            symbol
        )
        .fetch_one(&self.pool)
        .await?;

        for kline in klines {
            sqlx::query!(
		"INSERT INTO daily_ohlcv(coin_id, opentime, openprice, highprice, lowprice, closeprice, volume) VALUES ($1, $2, $3, $4, $5, $6, $7)",		
		symbol_id.id, kline.opentime, kline.open, kline.high, kline.low, kline.close, kline.volume)
		.execute(&self.pool)
		.await?;
        }

        Ok(())
    }
}

pub async fn setup_database() -> Db {
    let database_url = std::env::var("DATABASE_URL")
        .expect("ERROR: Environment variable \"DATABASE_URL\" was not found!");
    let res = Db::new(&database_url).await;

    match res {
        Ok(db) => db,
        Err(error) => match error {
            sqlx::Error::Configuration(s) => {
                eprintln!("ERROR:{}\tDatabase URL wrongly configurated", s);
                exit(-1);
            }
            sqlx::Error::Database(e) => {
                eprintln!("ERROR:{}\tDatabase error", e);
                exit(-1);
            }
            sqlx::Error::Io(e) => {
                eprintln!("ERROR:{}", e);
                exit(-1);
            }
            _ => {
                eprintln!("Another error occured");
                exit(-1);
            }
        },
    }
}
