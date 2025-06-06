
use actix_web::{get,   Responder};

#[get("/home")]
async fn home()->impl Responder{
    let respose:&str="welcome to tokkenly server";
    respose
}
