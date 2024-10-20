mod health;

use actix_web::{App, HttpServer};
use health::healthz;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(healthz))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
