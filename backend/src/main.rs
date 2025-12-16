// ...existing code...
use actix_files::Files;
use actix_web::{App, HttpServer, middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    // Serve './pkg' relative to process cwd
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(Files::new("/", "./pkg").index_file("index.html"))
            .default_service(actix_web::web::route().to(|| actix_web::HttpResponse::Found().header("Location", "/").finish()))
    })
    .bind(&bind_addr)?
    .run()
    .await
}