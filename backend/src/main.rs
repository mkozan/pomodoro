use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use actix_files as fs;
use std::{env, path::PathBuf};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    // Try common locations for the wasm-pack output `pkg` directory.
    let candidates = ["./pkg", "../pkg"];
    let mut pkg_path = None;
    for c in &candidates {
        let p = PathBuf::from(c);
        if p.is_dir() {
            pkg_path = Some(p);
            break;
        }
    }
    let pkg_dir = match pkg_path {
        Some(p) => p,
        None => {
            eprintln!("ERROR: could not find 'pkg' directory. Checked ./pkg and ../pkg");
            // fallback to ./pkg so actix will produce a clear error in logs
            PathBuf::from("./pkg")
        }
    };

    println!("🚀 Pomodoro Web Sunucusu başlatılıyor...");
    println!("📁 Serving frontend from: {:?}", pkg_dir.canonicalize().unwrap_or(pkg_dir.clone()));
    println!("📱 Tarayıcıda aç: http://localhost:{}", port);

    let pkg_str = pkg_dir.to_string_lossy().to_string();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // API rotaları
            .route("/api/health", web::get().to(health_check))
            // Statik dosyaları serve et (CSS, JS, HTML, WASM)
            .service(
                fs::Files::new("/", pkg_str.clone())
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
