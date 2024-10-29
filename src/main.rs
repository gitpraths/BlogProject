use actix_web::{App, HttpServer};
use actix_files as fs;
mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./static"))
            .configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}