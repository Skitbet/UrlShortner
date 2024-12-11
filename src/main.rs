use std::env;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use crate::db::MongoDB;
use crate::models::state::AppState;

mod db;
mod models;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").unwrap_or_else(|_| {
        panic!("No Mongo URI found in environment variables. Please set MONGO_URI.");
    });

    let database = MongoDB::new(&mongo_uri, "url_shortner").await;

    let data = web::Data::new(AppState {
        database,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(routes::get_services())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
