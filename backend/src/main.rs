use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use serde_json::json;
use chrono::Utc;

mod api;
mod domain;
mod infrastructure;

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "cfp-backend",
        "timestamp": Utc::now().to_rfc3339()
    }))
}

async fn root() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "service": "CFP Backend API",
        "version": "0.1.0",
        "description": "Charitable Foundation Publications System",
        "timestamp": Utc::now().to_rfc3339(),
        "endpoints": {
            "health": "/health",
            "status": "/status",
            "api_root": "/api",
            "members": "/api/members",
            "authors": "/api/authors",
            "texts": "/api/texts",
            "downloads": "/api/downloads"
        }
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Database connection
    let database_url = "postgresql://adnan:shahzadma@localhost:5432/cfp_db";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to database");
    
    // Initialize repositories
    let member_repo = infrastructure::database::repositories::MemberRepository::new(pool.clone());
    let author_repo = infrastructure::database::repositories::AuthorRepository::new(pool.clone());
    let text_repo = infrastructure::database::repositories::TextRepository::new(pool.clone());
    
    println!("=========================================");
    println!("🚀 CFP Backend - WITH CORS SUPPORT!");
    println!("=========================================");
    println!("🔧 Loading configuration...");
    println!("✅ Configuration loaded");
    println!("🔌 Connecting to database...");
    println!("✅ Database connected to cfp_db!");
    println!("🌐 Server: http://0.0.0.0:3000");
    println!("✅ Health: http://localhost:3000/health");
    println!(" ✅ CORS: Enabled for all origins");
    println!("📚 API: http://localhost:3000/api/status");
    println!("👥 Members: http://localhost:3000/api/members");
    println!("✍️  Authors: http://localhost:3000/api/authors");
    println!("📝 Texts: http://localhost:3000/api/texts");
    println!("📥 Downloads: http://localhost:3000/api/downloads");
    println!("=========================================");
    
    HttpServer::new(move || {
        // Enable CORS for frontend
        let cors = Cors::default()
            .allow_any_origin()  // Allow all origins (for development)
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);
        
        App::new()
            // Add CORS middleware
            .wrap(cors)
            // Root routes
            .route("/", web::get().to(root))
            .route("/health", web::get().to(health))
            .route("/status", web::get().to(health))
            // Add pool for download handlers
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(member_repo.clone()))
            .app_data(web::Data::new(author_repo.clone()))
            .app_data(web::Data::new(text_repo.clone()))
            .configure(api::routes::api_routes)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
