use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> String {
    "Hello world!".to_string()
}