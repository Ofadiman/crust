use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize)]
struct Path {
    id: uuid::Uuid,
}

#[derive(Deserialize, Debug)]
struct Body {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
struct Response {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[actix_web::patch("/{id}")]
pub async fn main(
    path: actix_web::web::Path<Path>,
    body: actix_web::web::Json<Body>,
    pool: actix_web::web::Data<sqlx::PgPool>,
) -> impl actix_web::Responder {
    let update_user_by_id_query = sqlx::query_as::<_, Response>(
        r"
            update users
            set first_name = $1,
                last_name  = $2,
                email      = $3,
                password   = $4
            where users.id = $5
            returning *;
        ",
    )
    .bind(&body.first_name.clone().or(Some("default".to_string())))
    .bind(&body.last_name.clone().or(Some("default".to_string())))
    .bind(&body.email.clone().or(Some("default".to_string())))
    .bind(&body.password.clone().or(Some("default".to_string())))
    .bind(&path.id)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap();

    match update_user_by_id_query {
        None => HttpResponse::NotFound().finish(),
        Some(value) => HttpResponse::Ok().json(value),
    }
}
