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
        .build_query_as::<Response>()
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();

    match update_user_by_id_query {
        None => HttpResponse::NotFound().finish(),
        Some(value) => HttpResponse::Ok().json(value),
    }
}
