mod health;
mod users_domain;
mod users_handle_create_user;
mod users_handle_get_user_by_id;
mod users_handle_paginate_users;
mod users_handle_update_user_by_id;

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
            .service(users_handle_get_user_by_id::handle_get_user_by_id)
            .service(users_handle_paginate_users::handle_paginate_users)
            .service(users_handle_create_user::handle_create_user)
            .service(users_handle_update_user_by_id::handle_update_user_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
