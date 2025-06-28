use actix_web::{App, HttpServer};
mod routes;
use routes::*;
mod database;
use database::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database= database_connection().await.expect("Failed to connect to database");


    println!("Database connection established");
    let server = HttpServer::new(move || {
        App::new()
        .app_data(database.clone())
            .service(home)
            .service(hello_user)
            .service(create_user)
            .service(register)
            .service(login)
    })
    .bind(("127.0.0.1", 8000))?
    .run();
    println!("Server is Running at 127.0.0:8000");
    server.await
}
