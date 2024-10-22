mod health;
mod udemy;
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
    let mut initial_users = Vec::new();

    initial_users.push(users_domain::User {
        id: 0,
        username: "john".to_owned(),
        email: "john@example.com".to_owned(),
        password: "password".to_owned(),
    });
    initial_users.push(users_domain::User {
        id: 1,
        username: "mark".to_owned(),
        email: "mark@example.com".to_owned(),
        password: "password".to_owned(),
    });
    initial_users.push(users_domain::User {
        id: 2,
        username: "michael".to_owned(),
        email: "michael@example.com".to_owned(),
        password: "password".to_owned(),
    });

    let state = web::Data::new(State {
        users: Mutex::new(initial_users),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(healthz)
            .service(users_handle_get_user_by_id::handle_get_user_by_id)
            .service(users_handle_paginate_users::handle_paginate_users)
            .service(users_handle_create_user::handle_create_user)
            .service(users_handle_update_user_by_id::handle_update_user_by_id)
            .service(web::scope("/udemy").service(udemy::response_headers))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
