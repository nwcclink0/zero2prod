use actix_web::Responder;
use actix_web::HttpResponse;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
