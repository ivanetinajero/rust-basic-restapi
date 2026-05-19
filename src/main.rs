use axum::response::IntoResponse;
use ejemplo_web::users;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let port = 8080;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap_or_else(|e| {
        panic!("Failed to bind to port {}: {}", addr, e);
    });

    let router = axum::Router::new()
        .route("/",axum::routing::get(root_handler).post(post_root_handler))
        .route("/api/users", axum::routing::post(users::handlers::create_user).get(users::handlers::read_users))
        .route("/api/users/{user_id}", axum::routing::get(users::handlers::read_user));

    println!("Server running on http://{}", addr);
    axum::serve(listener, router).await.unwrap_or_else(|e| {
        panic!("Failed to start server: {}", e);
    });
}

async fn root_handler() -> impl IntoResponse {
    println!("Received request at /");
    "Hello, World!"
}

async fn post_root_handler() -> impl IntoResponse {
    println!("POST Received request at /");
    "Hello, World!"
}
