use axum::response::IntoResponse;
use ejemplo_web::{context::AppContext, users};
use sea_orm::Database;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //println!("Database URL: {}", database_url);

    let db_conn = Database::connect(database_url).await.expect("Failed to connect to database");

    let ctx = AppContext {
        conn: db_conn,
        app_name: "Rust Basic REST API".to_string(),
        app_version: "0.1.0".to_string()
    };
    
    let port = 8080;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap_or_else(|e| {
        panic!("Failed to bind to port {}: {}", addr, e);
    });

    let router = axum::Router::new()
        .route("/",axum::routing::get(root_handler).post(post_root_handler))
        .route("/api/users", axum::routing::post(users::handlers::create_user).get(users::handlers::read_users))
        .route("/api/users/{user_id}", axum::routing::get(users::handlers::read_user))
        // Agrega el contexto a las rutas
        .with_state(ctx);

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
