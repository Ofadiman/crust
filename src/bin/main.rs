use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use crust::{
    health::healthz, settings::Settings, state::State, udemy, users_domain,
    users_handle_create_user, users_handle_get_user_by_id, users_handle_paginate_users,
    users_handle_update_user_by_id,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();

    let settings = Settings::new();
    println!("{settings:#?}");

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
            .service(
                web::scope("/udemy")
                    .service(udemy::response_headers)
                    .service(udemy::implement_responder)
                    .service(udemy::custom_errors)
                    .service(udemy::redirect),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
