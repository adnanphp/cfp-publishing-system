use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde_json::json;
use cfp_backend::config::AppConfig;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "CFP Backend is running"
    }))
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("CFP Backend API v0.1.0")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 CFP Backend starting...");
    
    // Load configuration
    let config = match AppConfig::load() {
        Ok(cfg) => {
            println!("✅ Configuration loaded");
            cfg
        }
        Err(e) => {
            eprintln!("❌ Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("📊 Database URL: {}", config.database.url);
    println!("🔗 Redis URL: {}", config.redis.url);
    
    // Clone for binding
    let bind_host = config.server.host.clone();
    let bind_port = config.server.port;
    
    // Try database connection
    match cfp_backend::infrastructure::database::create_pool(&config.database.url).await {
        Ok(_) => println!("✅ Database connection initialized"),
        Err(e) => println!("⚠️  Database connection failed: {}", e),
    }
    
    println!("🌐 Server listening on http://{}:{}", bind_host, bind_port);
    println!("✅ Health check: http://localhost:{}/health", bind_port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            .route("/api/test", web::get().to(|| async { "API endpoint" }))
            .route("/api/config", web::get().to(|config: web::Data<AppConfig>| async move {
                HttpResponse::Ok().json(json!({
                    "host": config.server.host,
                    "port": config.server.port,
                    "database": config.database.url,
                    "redis": config.redis.url,
                    "status": "running"
                }))
            }))
    })
    .bind(format!("{}:{}", bind_host, bind_port))?
    .run()
    .await
}
