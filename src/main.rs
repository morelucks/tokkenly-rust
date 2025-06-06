use actix_web::{get,  web::{self, Json, Path}, App, HttpServer, http::StatusCode, Responder};
use serde::{Serialize, Deserialize};

#[get("/home")]
async fn home()->impl Responder{
    let respose:&str="welcome to tokkenly server";
    respose
}


#[get("/hello/{fname}/{lname}/{email}")]
async fn hello_user(params: Path<(String, String, String)>) -> impl Responder {
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
    fn new(firstname:String, email:String, lastname:String)->Self{
        Self{
            first_name:firstname,
            last_name:lastname,
            email:email,
        }
    }
}

#[actix_web::main]  
async  fn main()->std::io::Result<()> {
    let server= HttpServer::new(||{
        App::new().service(home).service(hello_user)

    }).bind(("127.0.0.1", 8000))?
    .run();
    println!("Server is Running at 127.0.0:8000");
    server.await
}
