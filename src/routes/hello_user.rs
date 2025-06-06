
use actix_web::{get,  web::{Json, Path},  http::StatusCode, Responder};
use serde::{Serialize};

#[get("/hello/{fname}/{lname}/{email}")]
pub async fn hello_user(params: Path<(String, String, String)>) -> impl Responder {
    let (fname, lname, email) = params.into_inner();
    let response = User_details::new(fname, lname, email);
    (Json(response), StatusCode::OK)
}
#[derive(Serialize)]
struct User_details{
    first_name:String,
    last_name:String,
    email:String,
}
impl User_details {
    fn new(firstname:String, lastname:String, email:String )->Self{
        Self{
            first_name:firstname,
            last_name:lastname,
            email:email,
        }
    }
}