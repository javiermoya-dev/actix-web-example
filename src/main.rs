use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

// Define a simple struct to represent your data
#[derive(Debug, Serialize)]
struct Data {
    message: &'static str,
}

// Define a handler function for the root endpoint
#[get("/")]
async fn index() -> impl Responder {
    web::Json(Data {
        message: "Hello, Actix Web!",
    })
}

// Define another handler function for a different endpoint
#[get("/another")]
async fn another() -> impl Responder {
    web::Json(Data {
        message: "Another endpoint!",
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the Actix Web server
    HttpServer::new(|| {
        // Create an App and configure routes
        App::new()
            .service(index)
            .service(another)
    })
    .bind("127.0.0.1:8080")? // Bind to localhost on port 8080
    .run()
    .await
}
