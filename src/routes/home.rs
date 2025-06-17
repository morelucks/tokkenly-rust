use crate::routes::logging;
use actix_web::{Responder, get};

#[get("/home")]
async fn home() -> impl Responder {
    logging("GET: /home");
    let respose: &str = "welcome to tokkenly server";
    respose
}
