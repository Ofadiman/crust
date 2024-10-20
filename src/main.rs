mod health;
mod users;

use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use health::healthz;
use users::User;

pub struct State {
    users: Mutex<Vec<User>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(State {
        users: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(healthz)
            .service(users::get_user_by_id)
            .service(users::create_user)
            .service(users::get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
