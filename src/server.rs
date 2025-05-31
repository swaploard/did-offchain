use std::net::TcpListener;
use sqlx::PgPool;

pub fn get_tcp_listener() -> std::io::Result<TcpListener> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);

    let addr = format!("{}:{}", host, port);
    println!("🔧 Binding to address: {}", addr);

    // connection to the database
    connectin_to_database();
    TcpListener::bind(&addr)
}


fn connectin_to_database() {
    std::thread::spawn(|| {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

        runtime.block_on(async {
            let database_url = std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set");

            match PgPool::connect(&database_url).await {
                Ok(_) => println!("✅ Connected to PostgreSQL database"),
                Err(e) => eprintln!("❌ Failed to connect to PostgreSQL: {}", e),
            }
        });
    });
}
