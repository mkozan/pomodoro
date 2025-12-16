use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use actix_files as fs;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);
    
    println!("🚀 Pomodoro Web Sunucusu başlatılıyor...");
    println!("📱 Tarayıcıda aç: http://localhost:{}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            // API rotaları
            .route("/api/health", web::get().to(health_check))
            // Statik dosyaları serve et (CSS, JS, HTML, WASM)
            .service(
                fs::Files::new("/", "../pkg")
                    .index_file("index.html")
                    .use_last_modified(true)
                    .use_etag(true)
            )
    })
    .bind(&bind_addr)?
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
