use actix_web::{App, HttpServer, web, dev::Server};
use std::net::TcpListener;

use crate::routes;

pub fn get_tcp_listener() -> std::io::Result<TcpListener> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);

    let addr = format!("{}:{}", host, port);
    TcpListener::bind(&addr)
}

// pub fn run(listener: TcpListener) -> std::io::Result<Server> {
//     println!("Server running at: {:?}", listener.local_addr()?);

//     let server = HttpServer::new(|| {
//         App::new()
//             .service(web::scope("/api").configure(routes::user::configure))
//     })
//     .listen(listener)?
//     .run();

//     Ok(server)
// }
