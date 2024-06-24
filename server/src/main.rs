fn main() {
    let server = Server::new("localhost:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Server { addr }
    }

    fn run(self) {}
}
