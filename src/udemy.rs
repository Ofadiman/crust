use actix_web::{
    body::BoxBody,
    get,
    http::{
        header::{ContentType, HeaderName, HeaderValue},
        StatusCode,
    },
    web::{self, Redirect},
    HttpResponse, Responder, ResponseError,
};
use derive_more::derive::{Display, Error};
use serde::{Deserialize, Serialize};

#[get("/response-headers")]
pub async fn response_headers() -> impl Responder {
    if rand::random() {
        return HttpResponse::Ok()
            .insert_header(("x-programming-Language", "rust"))
            .finish();
    } else {
        let mut response = HttpResponse::new(StatusCode::OK);

        let headers = response.headers_mut();
        headers.append(
            HeaderName::from_static("x-response-from"),
            HeaderValue::from_static("ofadiman"),
        );

        return response;
    }
}

#[derive(Serialize)]
struct Pet {
    name: String,
}

impl Responder for Pet {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let s = serde_json::to_string(&self);
        match s {
            Ok(value) => HttpResponse::Ok().json(value),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[get("/implement-responder")]
pub async fn implement_responder() -> Pet {
    Pet {
        name: "max".to_string(),
    }
}

#[derive(Display, Error, Debug)]
pub enum CrustError {
    #[display("validation error on {field} field")]
    ValidationError { field: String },
    #[display("internal error")]
    InternalError,
    #[display("defensive error")]
    DefensiveError,
}

impl ResponseError for CrustError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DefensiveError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        return HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(self.to_string());
    }
}

#[derive(Debug, Deserialize)]
struct Query {
    r#type: String,
}

#[get("/custom-errors")]
pub async fn custom_errors(query: web::Query<Query>) -> Result<String, actix_web::error::Error> {
    if query.r#type == "validation".to_string() {
        return Err(CrustError::ValidationError {
            field: "name".to_string(),
        }
        .into());
    }

    if query.r#type == "defensive".to_string() {
        return Err(CrustError::DefensiveError.into());
    }

    return Err(CrustError::InternalError {}.into());
}

#[get("/redirect")]
pub async fn redirect() -> impl Responder {
    Redirect::to("/udemy/custom-errors?type=defensive")
}
