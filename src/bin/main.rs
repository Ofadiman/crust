use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crust::{health::healthz, settings::Settings, udemy, users};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    prelude::FromRow,
    types::Uuid,
};
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize, FromRow)]
struct SqlxQuery {
    id: i64,
    uuid: Uuid,
}

#[actix_web::get("/sqlx-query")]
async fn sqlx_query(pool: actix_web::web::Data<sqlx::PgPool>) -> impl Responder {
    let sqlx_query =
        sqlx::query_as::<_, SqlxQuery>("select 1::bigint as id, gen_random_uuid() as uuid;")
            .fetch_optional(pool.get_ref())
            .await
            .unwrap();

    HttpResponse::Ok().json(sqlx_query)
}

#[derive(Serialize, Debug)]
struct InvalidJsonError {
    pub message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();

    let settings = Settings::new();

    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(5)
        .idle_timeout(Duration::from_secs(300))
        .connect_with(
            PgConnectOptions::new()
                .username(&settings.postgres.username)
                .password(&settings.postgres.password)
                .host(&settings.postgres.host)
                .port(settings.postgres.port)
                .database(&settings.postgres.database),
        )
        .await
        .unwrap();

    let db = web::Data::new(pool);

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default().error_handler(|err, _req| {
            actix_web::error::InternalError::from_response(
                "cause: d143e2e1-b81e-43a3-930a-ff3b1f8fa24d",
                HttpResponse::BadRequest().json(InvalidJsonError {
                    message: err.to_string(),
                }),
            )
            .into()
        });

        let path_config = web::PathConfig::default().error_handler(|err, _req| {
            actix_web::error::InternalError::from_response(
                "cause: 47c72a82-d593-46c2-aa68-6919e17f16da",
                HttpResponse::BadRequest().json(InvalidJsonError {
                    message: err.to_string(),
                }),
            )
            .into()
        });

        let query_config = web::QueryConfig::default().error_handler(|err, _req| {
            actix_web::error::InternalError::from_response(
                "cause: 8521ec1e-4065-4b5f-8c07-41371e66e578",
                HttpResponse::BadRequest().json(InvalidJsonError {
                    message: err.to_string(),
                }),
            )
            .into()
        });

        App::new()
            .app_data(db.clone())
            .app_data(json_config)
            .app_data(path_config)
            .app_data(query_config)
            .service(sqlx_query)
            .service(healthz)
            .service(
                web::scope("/users")
                    .service(users::paginate::main)
                    .service(users::delete_by_id::main)
                    .service(users::create::main)
                    .service(users::get_by_id::main)
                    .service(users::update_by_id::main),
            )
            .service(
                web::scope("/udemy")
                    .service(udemy::response_headers)
                    .service(udemy::implement_responder)
                    .service(udemy::custom_errors)
                    .service(udemy::redirect),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
