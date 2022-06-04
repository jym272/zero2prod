use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, Responder};

async fn health_check() -> impl Responder {
    "Healthy"
}

// #[get("/health_check/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     format!("Hello {name}!")
// }

pub fn run(address: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().route("/health_check", web::get().to(health_check))
        // .service(greet)
    })
    .bind(address)?
    .run();
    Ok(server)
}