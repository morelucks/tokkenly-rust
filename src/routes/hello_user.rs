use actix_web::{
    Responder, get,
    http::StatusCode,
    web::{Json, Path},
};
use serde::{Deserialize, Serialize};

#[get("/hello/{fname}/{lname}/{email}")]
pub async fn hello_user(params: Path<(String, String, String)>) -> impl Responder {
    let (fname, lname, email) = params.into_inner();
    let response = UserDetails::new(fname, lname, email);
    (Json(response), StatusCode::OK)
}
#[derive(Serialize, Deserialize)]
pub struct UserDetails {
    first_name: String,
    last_name: String,
    email: String,
}
impl UserDetails {
    fn new(firstname: String, lastname: String, email: String) -> Self {
        Self {
            first_name: firstname,
            last_name: lastname,
            email: email,
        }
    }
}
