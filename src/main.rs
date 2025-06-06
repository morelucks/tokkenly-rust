use actix_web::{get,  web, App, HttpServer, web::Path, Responder};

#[get("/home")]
async fn home()->impl Responder{
    let respose:&str="welcome to tokkenly server";
    respose
}

#[get("/hello/{fname}/{lname}")]

async fn hello_user(params:Path<(String, String)>)->impl Responder{
    let respose=format!("hello {} {}", params.0, params.1);
    respose
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
