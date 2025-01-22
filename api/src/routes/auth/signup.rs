use actix_web::{post, Responder};

#[post("signup")]
async fn signup() -> impl Responder {}
