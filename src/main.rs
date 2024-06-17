use reqwest;

pub mod binance;

async fn get_klines(symbol: &str, limit: i32) -> Result<Vec<Kline>, reqwest::Error> {
    let res = reqwest::get(format!(
        "https://fapi.binance.com/fapi/v1/klines?symbol={symbol}&interval=1d&limit={limit}"
    ))
    .await?;

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

#[derive(Debug)]
struct Perpetual {
    symbol: String,
}

#[derive(Debug)]
struct Kline {
    opentime: i64,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
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
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let active_perps = get_active_perps().await?;

    if active_perps.len() > 0 {
        println!("{:?}", &active_perps[0].symbol);
        let klines = get_klines(&active_perps[0].symbol, 1).await;
        println!("{:?}", klines);
    }
    Ok(())
}
