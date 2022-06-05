use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    //    HttpResponse::Ok().finish()
    "Healthy"
}

#[derive(serde::Deserialize)]
struct Subscription {
    name: String,
    email: String,
}
//subscribe is not invoked until Deserialize succeeds with the form_data!!!
//it it fails it returns a 400 Bad Request, if it succeeds continues with the code in subscribe
async fn subscribe(form_data: web::Form<Subscription>) -> impl Responder {
    // format!("{}", form_data.email)
    HttpResponse::Ok().finish()
}

// #[get("/health_check/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     format!("Hello {name}!")
// }

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))

        // .service(greet)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub fn spawn_app() -> String {
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    //port 0: random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to start server.");
    let _ = tokio::spawn(server);
    //it's dropped when the tokio runtime is over, new runtime at the
    //beginning of each test
    format!("http://localhost:{}", port)
}
