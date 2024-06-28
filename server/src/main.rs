mod http;
mod server;
use http::request::Request;
use server::Server;

fn main() {
    let server = Server::new("localhost:8080".to_string());
    server.run();
}
