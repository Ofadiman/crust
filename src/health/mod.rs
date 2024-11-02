use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(tag = "Health", operation_id = "healthz", responses((status = 200, body = String)))]
#[get("/healthz")]
pub async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("ok")
}
