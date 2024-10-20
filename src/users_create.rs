use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{users_domain, State};

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserDto {
    username: String,
    email: String,
    password: String,
}

#[post("/users")]
pub async fn handle_create_user(
    body: web::Json<CreateUserDto>,
    data: web::Data<State>,
) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let new_user = users_domain::User {
        id: users.len() as u32,
        username: body.username.to_owned(),
        password: body.password.to_owned(),
        email: body.email.to_owned(),
    };
    users.push(new_user.clone());

    HttpResponse::Created().json(new_user)
}
