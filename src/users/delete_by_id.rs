use actix_web::{delete, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams)]
struct DeleteUserByIdPath {
    id: uuid::Uuid,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
struct DeleteUserByIdResponseBody {
    id: uuid::Uuid,
}

#[utoipa::path(tag = "Users", operation_id = "delete_user_by_id", params(DeleteUserByIdPath), responses((status = 200, description = "Successfully deleted user.", body = DeleteUserByIdResponseBody), (status = 404, description = "User not found.")))]
#[delete("/{id}")]
pub async fn main(
    path: actix_web::web::Path<DeleteUserByIdPath>,
    pool: actix_web::web::Data<sqlx::PgPool>,
) -> impl actix_web::Responder {
    let delete_user_query_result: Vec<DeleteUserByIdResponseBody> = sqlx::query_as(
        r"
            delete
            from users
            where users.id = $1
            returning id;
        ",
    )
    .bind(&path.id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    let maybe_deleted_user = delete_user_query_result.get(0);
    match maybe_deleted_user {
        None => HttpResponse::NotFound().finish(),
        Some(x) => HttpResponse::Ok().json(x),
    }
}
