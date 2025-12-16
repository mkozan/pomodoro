use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use actix_files as fs;
use std::path::PathBuf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Pomodoro Web Sunucusu başlatılıyor...");
    println!("📱 Tarayıcıda aç: http://localhost:8000");

    // Frontend dizini
    let frontend_path = PathBuf::from("../pkg");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // API rotaları
            .route("/api/health", web::get().to(health_check))
            // Statik dosyaları serve et (CSS, JS, HTML, WASM)
            .service(
                fs::Files::new("/", &frontend_path)
                    .index_file("index.html")
                    .use_last_modified(true)
                    .use_etag(true)
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

// Basit health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Pomodoro sunucusu çalışıyor"
    }))
}
