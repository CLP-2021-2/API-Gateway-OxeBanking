use actix_web::{get, web, App, HttpServer, Responder};
mod sample; 

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(sample::test)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}