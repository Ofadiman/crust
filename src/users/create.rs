use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::chrono};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct Body {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, FromRow)]
struct Response {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[actix_web::post("")]
pub async fn main(
    body: actix_web::web::Json<Body>,
    pool: actix_web::web::Data<sqlx::PgPool>,
) -> impl actix_web::Responder {
    let insert_user_query = sqlx::query_as::<_, Response>(
        r"
            insert into users (first_name, last_name, email, password)
            values ($1, $2, $3, $4)
            returning *;
        ",
    )
    .bind(&body.first_name)
    .bind(&body.last_name)
    .bind(&body.email)
    .bind(&body.password)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap();

    match insert_user_query {
        None => HttpResponse::InternalServerError().finish(),
        Some(value) => HttpResponse::Ok().json(value),
    }
}
