use actix_web::{
    body::BoxBody,
    get,
    http::{
        header::{HeaderName, HeaderValue},
        StatusCode,
    },
    HttpResponse, Responder,
};
use serde::Serialize;

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
