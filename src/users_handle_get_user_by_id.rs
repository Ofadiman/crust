use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::State;

#[derive(Debug, Deserialize)]
struct Path {
    user_id: u32,
}

#[get("/users/{user_id}")]
pub async fn handle_get_user_by_id(
    path: web::Path<Path>,
    data: web::Data<State>,
) -> impl Responder {
    let users = data.users.lock().unwrap();

    let maybe_user = users.iter().find(|user| user.id == path.user_id);

    if let Some(user) = maybe_user {
        return HttpResponse::Ok().json(user);
    }

    HttpResponse::NotFound().finish()
}
