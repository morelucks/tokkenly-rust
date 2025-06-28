use crate::routes::logging;
use actix_web::{Responder, get};

#[get("/home")]
async fn home() -> impl Responder {
    logging("GET: /home");
    let response: &str = "welcome to tokkenly server";
    response
}
