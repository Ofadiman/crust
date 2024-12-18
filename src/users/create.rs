use actix_web::post;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::chrono};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::settings;
use crate::utils;
use crate::utils::passwords::PasswordManager;

#[derive(Debug, Validate, Deserialize, ToSchema)]
struct CreateUserRequestBody {
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

#[derive(Debug, Serialize, FromRow, ToSchema)]
struct CreateUserResponseBody {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[utoipa::path(tag = "Users", operation_id = "create_user", responses((status = 201, body = CreateUserResponseBody)))]
#[post("")]
pub async fn main(
    body: actix_web::web::Json<CreateUserRequestBody>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    password_manager: actix_web::web::Data<utils::passwords::PasswordManager<'_>>,
    settings: actix_web::web::Data<settings::Settings>,
) -> impl actix_web::Responder {
    let validation_result = body.validate();
    if let Err(value) = validation_result {
        return HttpResponse::BadRequest().json(value);
    }

    let hashed_password = password_manager.hash(&body.password, &settings.passwords.pepper);

    let insert_user_query = sqlx::query_as::<_, CreateUserResponseBody>(
        r"
            insert into users (first_name, last_name, email, password)
            values ($1, $2, $3, $4)
            returning *;
        ",
    )
    .bind(&body.first_name)
    .bind(&body.last_name)
    .bind(&body.email)
    .bind(hashed_password)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap();

    match insert_user_query {
        None => HttpResponse::InternalServerError().finish(),
        Some(value) => HttpResponse::Ok().json(value),
    }
}
