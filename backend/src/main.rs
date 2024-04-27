use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello =
        Router::new().route("/hello", get(|| async { Html("<h1>Hello world</h1>") }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Listening on {:?}", listener.local_addr());
    axum::serve(listener, routes_hello).await.unwrap();
}
