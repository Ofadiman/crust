use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize)]
struct Path {
    id: uuid::Uuid,
}

#[derive(Debug, Serialize, FromRow)]
struct QueryResult {
    id: uuid::Uuid,
}

#[actix_web::delete("{id}")]
pub async fn main(
    path: actix_web::web::Path<Path>,
    pool: actix_web::web::Data<sqlx::PgPool>,
) -> impl actix_web::Responder {
    let delete_user_query: Vec<QueryResult> = sqlx::query_as(
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

    println!("{delete_user_query:#?}");

    if delete_user_query.len() == 0 {
        return HttpResponse::NotFound().finish();
    }

    HttpResponse::NoContent().finish()
}
