use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(_from: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
