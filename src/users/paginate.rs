use actix_web::HttpResponse;
use derive_more::derive::{Debug, Display};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Deserialize, Debug, Display)]
enum OrderDirection {
    #[serde(rename(deserialize = "asc"))]
    #[display("asc")]
    Ascending,
    #[serde(rename(deserialize = "desc"))]
    #[display("desc")]
    Descending,
}

#[derive(Deserialize, Debug, Display)]
enum OrderBy {
    #[serde(rename(deserialize = "first_name"))]
    #[display("first_name")]
    FirstName,
    #[serde(rename(deserialize = "last_name"))]
    #[display("last_name")]
    LastName,
    #[serde(rename(deserialize = "id"))]
    #[display("id")]
    Id,
}

#[derive(Deserialize, Validate, Debug, Display)]
#[display(
    "offset: {offset}, limit: {limit}, order_by: {order_by}, order_direction: {order_direction}"
)]
struct Query {
    offset: i32,
    limit: i32,
    order_by: OrderBy,
    order_direction: OrderDirection,
}

#[derive(Serialize, Debug, FromRow)]
struct Response {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[actix_web::get("")]
pub async fn main(
    query: actix_web::web::Query<Query>,
    pool: actix_web::web::Data<sqlx::PgPool>,
) -> impl actix_web::Responder {
    let mut query_builder =
        sqlx::QueryBuilder::<sqlx::Postgres>::new("select * from users order by ");

    query_builder.push_bind(query.order_by.to_string());
    query_builder.push(" ");
    query_builder.push(query.order_direction.to_string());
    query_builder.push(" offset ");
    query_builder.push_bind(&query.offset);
    query_builder.push(" limit ");
    query_builder.push_bind(&query.limit);
    query_builder.push(";");
    println!("{}", query_builder.sql());

    let paginate_users_query: Vec<Response> = query_builder
        .build_query_as()
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(paginate_users_query)
}
