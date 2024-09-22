#[path = "api/routing.rs"]
pub mod routing;

use axum::Router;
use std::path::PathBuf;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    const SECURE: bool = false;
    let socket = ":::3000";
    let build_dir = PathBuf::from("build");
    let app = Router::new()
        .nest("/api/", routing::api_routes())
        .fallback_service(
            ServeDir::new(build_dir.clone())
                .not_found_service(ServeFile::new(build_dir.join("app.html"))),
        );

    let addr = tokio::net::TcpListener::bind(socket).await.unwrap();
    println!(
        "Listening at {}://{}",
        if SECURE { "https" } else { "http" },
        socket
    );
    axum::serve(addr, app.into_make_service()).await.unwrap();
}
