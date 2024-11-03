use actix_web::cookie::Cookie;
use actix_web::post;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::settings;
use crate::utils;

#[derive(Debug, Validate, Deserialize, ToSchema)]
struct LoginRequestBody {
    #[validate(
        email,
        length(min = 1, max = 50, message = "minimum or maximum length exceeded")
    )]
    email: String,
    #[validate(length(min = 1, max = 50, message = "minimum or maximum length exceeded"))]
    password: String,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
struct UserByIdQueryResult {
    id: Uuid,
    password: String,
}

#[utoipa::path(tag = "Users", operation_id = "login", responses((status = 200)))]
#[post("/login")]
pub async fn main(
    body: actix_web::web::Json<LoginRequestBody>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    password_manager: actix_web::web::Data<utils::passwords::PasswordManager<'_>>,
    settings: actix_web::web::Data<settings::Settings>,
) -> impl actix_web::Responder {
    let validation_result = body.validate();
    if let Err(value) = validation_result {
        return HttpResponse::BadRequest().json(value);
    }

    let select_user_by_email_query = sqlx::query_as::<_, UserByIdQueryResult>(
        r"
            select users.id, users.password from users where users.email = $1;
        ",
    )
    .bind(&body.email)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap();

    if let Some(user) = select_user_by_email_query {
        let is_match =
            password_manager.compare(&body.password, &settings.passwords.pepper, &user.password);

        if is_match {
            let session_cookie = Cookie::build("session_id", uuid::Uuid::new_v4().to_string())
                .domain("localhost")
                .path("/")
                .secure(true)
                .http_only(true)
                .finish();

            HttpResponse::Ok().cookie(session_cookie).json(json!({
                "code": 200,
                "success": true,
                "payload": {
                    "features": [
                        "serde",
                        "json"
                    ],
                    "homepage": null
                }
            }))
        } else {
            HttpResponse::NotFound().json(json!({
                "code": 404,
                "success": false,
            }))
        }
    } else {
        HttpResponse::NotFound().json(json!({
            "code": 404,
            "success": false,
        }))
    }
}
