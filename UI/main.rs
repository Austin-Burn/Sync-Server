
use axum::{
    routing::get,
    Router,
};
use tokio;

async fn hello_world() -> &'static str {
    println!("Hello, World!");
    "Hello, World!"
}
async fn ui_endpoint() -> &'static str {
    "ui connection point"
}

async fn button_endpoint() -> &'static str {
    "button connection point"
}
async fn http_server() {
    // Build our application with a route
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/ui", get(ui_endpoint));

    // Run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("HTTP server listening on 127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}


#[tokio::main]
async fn main() {
    // Start HTTP server in a separate thread
    let http_handle = tokio::spawn(http_server());

    // Wait for both servers to finish (though they never will in this case)
    let _ = tokio::join!(http_handle);

}

