use actix_web::{get, web, HttpResponse, Responder};

use crate::State;

#[get("/users")]
pub async fn handle_paginate_users(data: web::Data<State>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(users.clone())
}
