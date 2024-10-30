use actix_web::HttpResponse;
use derive_more::derive::Debug;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Debug)]
struct Path {
    id: uuid::Uuid,
}

#[derive(Serialize, Debug, FromRow)]
struct Response {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[actix_web::get("/{id}")]
pub async fn main(
    path: actix_web::web::Path<Path>,
    pool: actix_web::web::Data<sqlx::PgPool>,
) -> impl actix_web::Responder {
    let select_user_by_id_query = sqlx::query_as::<_, Response>(
        r"
            select *
            from users
            where users.id = $1;
        ",
    )
    .bind(&path.id)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap();

    match select_user_by_id_query {
        None => HttpResponse::NotFound().finish(),
        Some(user) => HttpResponse::Ok().json(user),
    }
}
