use dotenv::dotenv;

use std::{env, process::exit};
use thiserror::Error;

pub mod binance;
pub mod common;
pub mod database;

use database::setup_database;

fn print_help() {
    println!("Crypto Trendfollowing Tool");
    println!("Commands are:");
    println!("--setup\tChecks and then performs an interactive setup of the database");
    println!("--prod\tRuns in production mode. Fails if checks fails");
    println!("--check\tPerforms check to make sure everything is all right");
    println!("--help\tDisplays help");
}

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("DATABASE_URL environment variable is not set")]
    DatabaseUrlNotSet,

    #[error("DATABASE_URL is not well-formed for PostgreSQL")]
    DatabaseUrlMalformed,

    #[error("Failed to connect to the database: {0}")]
    ConnectionError(#[from] sqlx::Error),

    #[error("Table `{0}` does not exist")]
    TableDoesNotExist(String),

    #[error("Table `{0}` does not have an `{1}` column")]
    ColumnDoesNotExist(String, String),
}

async fn checking() -> Result<(), SetupError> {
    let database_url = env::var("DATABASE_URL").map_err(|_| SetupError::DatabaseUrlNotSet)?;
    println!("DATABASE_URL set! ✓");

    if !database_url.starts_with("postgres://") {
        return Err(SetupError::DatabaseUrlMalformed);
    }
    println!("URL set for postgresql! ✓");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .map_err(SetupError::ConnectionError)?;
    println!("DB connection established succesfully! ✓");

    let required_tables = vec!["perpetual_futures", "daily_ohlcv"];

    for table in &required_tables {
        let result: (i64,) = sqlx::query_as(&format!(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = '{}'",
            table
        ))
        .fetch_one(&pool)
        .await
        .map_err(SetupError::ConnectionError)?;

        if result.0 == 0 {
            return Err(SetupError::TableDoesNotExist(table.to_string()));
        }
    }

    println!("Database setup is correct.");
    Ok(())
}

async fn setup() -> Result<(), SetupError> {
    let database_url = env::var("DATABASE_URL").map_err(|_| SetupError::DatabaseUrlNotSet)?;
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .map_err(SetupError::ConnectionError)?;

    println!("CREATING TABLES");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS perpetual_futures(
       id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
       symbol TEXT NOT NULL
);",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS daily_ohlcv(
       coin_id BIGINT references perpetual_futures(id),
       opentime BIGINT NOT NULL,
       openprice TEXT NOT NULL,
       highprice TEXT NOT NULL,
       lowprice TEXT NOT NULL,
       closeprice TEXT NOT NULL,
       volume TEXT NOT NULL
);",
    )
    .execute(&pool)
    .await?;

    Ok(())
}
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_help();
        exit(0);
    }

    dotenv().ok();

    match args[1].as_str() {
        "--setup" => {
            println!("SETUP");
            let res = setup().await;
            match res {
                Ok(_) => println!("Setup successful!"),
                Err(e) => eprintln!("Setup failed: {:?}", e),
            }
        }
        "--prod" => {
            println!("PRODUCTION");
            let res = checking().await;
            match res {
                Ok(_) => println!("Running"),
                Err(e) => {
                    eprintln!("Check failed with: :\t{:?}", e);
                    exit(-1);
                }
            }
        }
        "--check" => {
            let res = checking().await;
            match res {
                Ok(_) => println!("OK"),
                Err(e) => eprintln!("ERROR:\t{:?}", e),
            }
        }
        _ => print_help(),
    }

    // let active_perps = get_active_perps().await?;

    // if active_perps.len() > 0 {
    //     println!("{:?}", &active_perps[0].symbol);
    //     let all_klines = scrape_all_klines("BTCUSDT").await;
    //     println!(
    //         "LENGTH:{}\t{:?}",
    //         all_klines.len(),
    //         all_klines.first().expect("No first element")
    //     );
    //     for i in 0..(all_klines.len() - 1) {
    //         if all_klines[i].opentime == all_klines[i + 1].opentime {
    //             println!("Double at {}", all_klines[i].opentime);
    //         }
    //     }
    // }
    // Ok(())
    dotenv().ok();
    let _db = setup_database().await;

    // let active_perps = get_active_perps().await;
    // match active_perps {
    //     Ok(perps) => {
    //         let _ = db.insert_perpetual_futures(perps).await;
    //         println!("done")
    //     }
    //     Err(e) => println!("{}", e),
    // }

    // let perps = db
    //     .get_all_perpetual_futures()
    //     .await
    //     .expect("Couldn't get the perps from the database");

    // for perp in perps {
    //     println!("{}", perp.symbol);
    //     let all_klines = scrape_all_klines(&perp.symbol).await;
    //     let res = db.insert_daily_klines(&perp.symbol, all_klines).await;
    //     match res {
    //         Ok(_) => println!("Insert klines for {}", &perp.symbol),
    //         Err(e) => println!("Something went wrong for {}\tERROR:{}", &perp.symbol, e),
    //     }
    // }
}
