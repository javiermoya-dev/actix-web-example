use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
/*use diesel::r2d2::{self, ConnectionManager};*/
use diesel::r2d2::ConnectionManager;
use r2d2::{self, PooledConnection};
use dotenv::dotenv;
use std::env;
use serde::Serialize;
use lazy_static::lazy_static;


mod db;
mod models;
mod schema;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Serialize)]
struct Data {
    message: &'static str,
}

#[get("/")]
async fn index() -> impl Responder {
    web::Json(Data {
        message: "Hello, Actix Web!",
    })
}

#[get("/items")]
async fn get_items() -> impl Responder {
    use crate::models::Item;

    match Item::all(&mut db::connection()) {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db::init().clone()))
            .service(index)
            .service(get_items)
            .wrap(Cors::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
