use actix_web::{
    Responder,
    http::StatusCode,
    post,
    web::Json,
};

use crate::routes::{UserDetails, logging};
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserRes {
    id: u32,
    user: UserDetails,
}

#[post("user/create")]
async fn create_user(user: Json<UserDetails>) -> impl Responder {
    logging("POST: /user/create");
    (
        Json(CreateUserRes {
            id: 1,
            user: user.0,
        }),
        StatusCode::CREATED,
    )
}
