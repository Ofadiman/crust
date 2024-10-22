use actix_web::{
    get,
    http::{
        header::{HeaderName, HeaderValue},
        StatusCode,
    },
    HttpResponse, Responder,
};

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
