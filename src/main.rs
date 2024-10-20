mod health;
mod users_create;
mod users_domain;
mod users_get_by_id;
mod users_paginate;

use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use health::healthz;

pub struct State {
    users: Mutex<Vec<users_domain::User>>,
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
            .service(users_get_by_id::handle_get_user_by_id)
            .service(users_create::handle_create_user)
            .service(users_paginate::handle_paginate_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
