use actix_web::{web, App, HttpResponse, HttpServer};
use crust::{health::healthz, settings::Settings, users};
use serde::Serialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::time::Duration;
use utoipa::OpenApi;
use utoipa_actix_web::{scope, AppExt};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Users", description = "Endpoints used to manage user resource."),
        (name = "Health", description = "Endpoints used to monitor application health."),
    ),
    components(schemas(users::paginate::PaginateUsersSortDirection, users::paginate::PaginateUsersSortField))
)]
pub struct ApiDoc;

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

        let (app, api) = App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .app_data(db.clone())
            .app_data(json_config)
            .app_data(path_config)
            .app_data(query_config)
            .service(healthz)
            .service(
                scope::scope("/users")
                    .service(users::paginate::main)
                    .service(users::delete_by_id::main)
                    .service(users::get_by_id::main)
                    .service(users::update_by_id::main)
                    .service(users::create::main),
            )
            .split_for_parts();

        app.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
