use crate::common::Perpetual;
use crate::database::{setup_database, Db};
use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

pub mod common;
pub mod database;

struct AppState {
    db: Db,
}

#[derive(Serialize, Deserialize)]
struct Error {
    msg: String,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let db = &data.db;
    let all_perps = db.get_all_perpetual_futures().await;

    match all_perps {
        Ok(perps) => {
            HttpResponse::Ok().json(perps.into_iter().map(|p| p.symbol).collect::<Vec<String>>())
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("{}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = setup_database().await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
