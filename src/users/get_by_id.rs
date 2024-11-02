use actix_web::{get, HttpResponse};
use derive_more::derive::Debug;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::IntoParams;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, IntoParams)]
struct GetUserByIdPath {
    id: uuid::Uuid,
}

#[derive(Serialize, Debug, FromRow, ToSchema)]
struct GetUserByIdResponseBody {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[utoipa::path(tag = "Users", operation_id = "get_user_by_id", params(GetUserByIdPath), responses((status = 200, body = GetUserByIdResponseBody, description = "Successfully retrieved user by id."), (status = 404, description = "User not found.")))]
#[get("/{id}")]
pub async fn main(
    path: actix_web::web::Path<GetUserByIdPath>,
    pool: actix_web::web::Data<sqlx::PgPool>,
) -> impl actix_web::Responder {
    let select_user_by_id_query_result = sqlx::query_as::<_, GetUserByIdResponseBody>(
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

    match select_user_by_id_query_result {
        None => HttpResponse::NotFound().finish(),
        Some(user) => HttpResponse::Ok().json(user),
    }
}
