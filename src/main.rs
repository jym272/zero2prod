use zero2prod::run;

#[tokio::main] // or #[tokio::main] //[actix_web::main]
async fn main() -> std::io::Result<()> {
    run("127.0.0.1:8080")?.await
}
