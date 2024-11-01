use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::chrono};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct Body {
    #[validate(length(min = 1, max = 50, message = "minimum or maximum length exceeded"))]
    first_name: String,
    #[validate(length(min = 1, max = 50, message = "minimum or maximum length exceeded"))]
    last_name: String,
    #[validate(
        email,
        length(min = 1, max = 50, message = "minimum or maximum length exceeded")
    )]
    email: String,
    #[validate(length(min = 1, max = 50, message = "minimum or maximum length exceeded"))]
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
    let validation_result = body.validate();
    if let Err(value) = validation_result {
        return HttpResponse::BadRequest().json(value);
    }

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
