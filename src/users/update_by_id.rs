use actix_web::{patch, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, IntoParams)]
struct UpdateUserByIdPath {
    id: uuid::Uuid,
}

#[derive(Deserialize, Validate, Debug, ToSchema)]
struct UpdateUserByIdRequestBody {
    #[validate(length(min = 1, max = 50, message = "minimum or maximum length exceeded"))]
    first_name: Option<String>,
    #[validate(length(min = 1, max = 50, message = "minimum or maximum length exceeded"))]
    last_name: Option<String>,
    #[validate(
        email,
        length(min = 1, max = 50, message = "minimum or maximum length exceeded")
    )]
    email: Option<String>,
    #[validate(length(min = 1, max = 50, message = "minimum or maximum length exceeded"))]
    password: Option<String>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
struct UpdateUserByIdResponseBody {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[utoipa::path(tag = "Users", operation_id = "update_user_by_id", responses((status = 200, body = UpdateUserByIdResponseBody, description = "Successfully updated user."), (status = 404, description = "User not found.")))]
#[patch("/{id}")]
pub async fn main(
    path: actix_web::web::Path<UpdateUserByIdPath>,
    body: actix_web::web::Json<UpdateUserByIdRequestBody>,
    pool: actix_web::web::Data<sqlx::PgPool>,
) -> impl actix_web::Responder {
    let validation_result = body.validate();
    if let Err(value) = validation_result {
        return HttpResponse::BadRequest().json(value);
    }

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("update users set ");

    let mut first = true;
    if let Some(first_name) = body.first_name.clone() {
        query_builder.push("first_name = ");
        query_builder.push_bind(first_name);
        first = false;
    }

    if let Some(last_name) = body.last_name.clone() {
        if !first {
            query_builder.push(", ");
        }
        query_builder.push("last_name = ");
        query_builder.push_bind(last_name);
        first = false;
    }

    if let Some(email) = body.email.clone() {
        if !first {
            query_builder.push(", ");
        }
        query_builder.push("email = ");
        query_builder.push_bind(email);
        first = false;
    }

    if let Some(password) = body.password.clone() {
        if !first {
            query_builder.push(", ");
        }
        query_builder.push("password = ");
        query_builder.push_bind(password);
    }

    if first {
        return HttpResponse::UnprocessableEntity().finish();
    }

    query_builder.push(" where id = ");
    query_builder.push_bind(path.id);
    query_builder.push(" returning *");

    let update_user_by_id_query = query_builder
        .build_query_as::<UpdateUserByIdResponseBody>()
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();

    match update_user_by_id_query {
        None => HttpResponse::NotFound().finish(),
        Some(value) => HttpResponse::Ok().json(value),
    }
}
