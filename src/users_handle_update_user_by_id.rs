use actix_web::{patch, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::State;

#[derive(Debug, Deserialize)]
struct Path {
    user_id: u32,
}

#[derive(Debug, Deserialize)]
struct Body {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

#[patch("/users/{user_id}")]
pub async fn handle_update_user_by_id(
    path: web::Path<Path>,
    data: web::Data<State>,
    body: web::Json<Body>,
) -> impl Responder {
    let mut users = data.users.lock().unwrap();

    if let Some(user) = users.iter_mut().find(|user| user.id == path.user_id) {
        if let Some(username) = body.username.as_ref() {
            user.username = username.clone();
        }
        if let Some(email) = body.email.as_ref() {
            user.email = email.clone();
        }
        if let Some(password) = body.password.as_ref() {
            user.password = password.clone();
        }

        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}
