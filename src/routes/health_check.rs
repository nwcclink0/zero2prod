use actix_web::HttpResponse;
use actix_web::Responder;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
