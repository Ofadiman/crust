use actix_web::get;
use actix_web::HttpResponse;
use derive_more::derive::{Debug, Display};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::IntoParams;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Debug, Display, ToSchema)]
pub enum PaginateUsersSortDirection {
    #[display("asc")]
    asc,
    #[display("desc")]
    desc,
}

#[derive(Deserialize, Debug, Display, ToSchema)]
pub enum PaginateUsersSortField {
    #[display("first_name")]
    first_name,
    #[display("last_name")]
    last_name,
    #[display("id")]
    id,
}

#[derive(Deserialize, Validate, Debug, Display, IntoParams, ToSchema)]
#[display(
    "offset: {offset}, limit: {limit}, sort_field: {sort_field}, sort_direction: {sort_direction}"
)]
struct PaginateUsersQuery {
    offset: i32,
    limit: i32,
    sort_field: PaginateUsersSortField,
    sort_direction: PaginateUsersSortDirection,
}

#[derive(Serialize, Debug, FromRow, ToSchema)]
struct PaginateUsersResponseBody {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[utoipa::path(tag = "Users", operation_id = "paginate_users", params(PaginateUsersQuery), responses((status = 200, body = PaginateUsersResponseBody, description = "Successfully paginated users.")))]
#[get("")]
pub async fn main(
    query: actix_web::web::Query<PaginateUsersQuery>,
    pool: actix_web::web::Data<sqlx::PgPool>,
) -> impl actix_web::Responder {
    let mut query_builder =
        sqlx::QueryBuilder::<sqlx::Postgres>::new("select * from users order by ");

    query_builder.push_bind(query.sort_field.to_string());
    query_builder.push(" ");
    query_builder.push(query.sort_direction.to_string());
    query_builder.push(" offset ");
    query_builder.push_bind(&query.offset);
    query_builder.push(" limit ");
    query_builder.push_bind(&query.limit);
    query_builder.push(";");
    println!("{}", query_builder.sql());

    let paginate_users_query: Vec<PaginateUsersResponseBody> = query_builder
        .build_query_as()
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(paginate_users_query)
}
