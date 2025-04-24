use axum::{routing::get, Router};
use std::net::UdpSocket;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn button_click() -> &'static str {
    println!("Button clicked!");
    "Button clicked!"
}

async fn send_video() -> &'static str {
    println!("Sending video!");
    "Sending video!"
}

async fn receive_video() -> &'static str {
    println!("Receiving video!");
    "Receiving video!"
}

#[tokio::main]
async fn main() {
    // Start UDP server
    //let udp_socket = UdpSocket::bind("127.0.0.1:8080").unwrap();
    //println!("UDP server listening on 127.0.0.1:8080");
    //tokio::spawn(async move {
    //    let mut buf = [0; 1024];
    //    loop {
    //        if let Ok((size, addr)) = udp_socket.recv_from(&mut buf) {
    //            udp_socket.send_to(b"Hello, World!", addr).unwrap();
    //        }
    //    }
    //});

    // Start HTTP server
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/ui/button", get(button_click))
        .nest_service("/ui", ServeDir::new("static"))
        .route("/receiveVideo", get(receive_video))
        .route("/sendVideo", get(send_video));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("HTTP server listening on 127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
