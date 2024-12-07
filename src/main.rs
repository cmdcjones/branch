use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));
    let port = "4000".to_string();
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .expect("Failed to bind to port!");
    println!("App started on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Failed to start server!");
}

async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}
