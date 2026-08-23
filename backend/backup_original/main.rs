use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde_json::json;
use cfp_backend::config::AppConfig;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "CFP Backend is running",
        "version": "1.0.0"
    }))
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
    <html>
        <head><title>CFP Backend</title></head>
        <body style="font-family: Arial, sans-serif; margin: 40px;">
            <h1>🚀 CFP Backend - Charity Funding Platform</h1>
            <p>Your project has been successfully rebuilt!</p>
            
            <h2>📊 Endpoints:</h2>
            <ul>
                <li><a href="/health">/health</a> - Health check</li>
                <li><a href="/api/test">/api/test</a> - API test</li>
                <li><a href="/api/status">/api/status</a> - System status</li>
            </ul>
            
            <h2>🏗️ Project Structure:</h2>
            <pre>
src/
├── main.rs          # Server entry point
├── lib.rs           # Module exports
├── config/          # Configuration
├── domain/          # Business logic
│   ├── enums/      # MemberStatus, TextStatus
│   └── models/     # Member, Text models
└── api/            # API layer
    └── routes/     # HTTP endpoints
            </pre>
            
            <h2>✅ Success!</h2>
            <p>Your CFP Backend is now running with a clean, working architecture.</p>
            <p>You can gradually add your original files back to this foundation.</p>
        </body>
    </html>
    "#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("=========================================");
    println!("🚀 CFP Backend - SUCCESSFULLY REBUILT!");
    println!("=========================================");
    
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
    
    let bind_host = config.server.host.clone();
    let bind_port = config.server.port;
    
    println!("🌐 Server: http://{}:{}", bind_host, bind_port);
    println!("✅ Health: http://localhost:{}/health", bind_port);
    println!("📚 API: http://localhost:{}/api/status", bind_port);
    println!("🌍 Web: http://localhost:{}/", bind_port);
    println!("=========================================");
    println!("");
    println!("🎉 CONGRATULATIONS! Your project is now WORKING!");
    println!("");
    println!("Next steps to restore your original code:");
    println!("1. Keep this working foundation");
    println!("2. Copy files from backup one at a time");
    println!("3. Fix imports gradually");
    println!("4. Test after each addition");
    println!("");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            .configure(cfp_backend::api::routes::configure_routes)
    })
    .bind(format!("{}:{}", bind_host, bind_port))?
    .run()
    .await
}
