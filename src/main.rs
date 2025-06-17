use actix_web::{App, HttpServer};
mod routes;
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(home)
            .service(hello_user)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8000))?
    .run();
    println!("Server is Running at 127.0.0:8000");
    server.await
}
