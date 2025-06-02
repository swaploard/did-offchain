use std::net::TcpListener;

pub fn get_tcp_listener() -> std::io::Result<TcpListener> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);

    let addr = format!("{}:{}", host, port);
    println!("🔧 Binding to address: {}", addr);

    TcpListener::bind(&addr)
}
