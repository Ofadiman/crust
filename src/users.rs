use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: u32,
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct Path {
    user_id: u32,
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(path: web::Path<Path>) -> impl Responder {
    let user = User {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    username: String,
    email: String,
    password: String,
}

#[post("/users")]
pub async fn create_user(body: web::Json<CreateUserDto>, data: web::Data<State>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let new_user = User {
        id: users.len() as u32,
        username: body.username.to_owned(),
        password: body.password.to_owned(),
        email: body.email.to_owned(),
    };
    users.push(new_user.clone());

    HttpResponse::Created().json(new_user)
}

#[get("/users")]
pub async fn get_users(data: web::Data<State>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(users.clone())
}
