use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::users_domain;

#[derive(Debug, Deserialize)]
struct Path {
    user_id: u32,
}

#[get("/users/{user_id}")]
pub async fn handle_get_user_by_id(path: web::Path<Path>) -> impl Responder {
    let user = users_domain::User {
        id: 1,
        username: "john".to_owned(),
        email: "john@example.com".to_owned(),
        password: "plain text password".to_owned(),
    };

    if path.user_id == 1 {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}
