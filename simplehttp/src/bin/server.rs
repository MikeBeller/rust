pub use env_logger;
use log::info;
use simple_server::{Method, Server, StatusCode};

fn main() {
    env_logger::init();

    let host = "0.0.0.0";
    let port = "7878";

    let server = Server::new(|request, mut response| {
        info!("Request received. {} {}", request.method(), request.uri());
        match (request.method(), request.uri().path()) {
            (&Method::GET, "/hello") => {
                Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>\n".as_bytes().to_vec())?)
            }
            (_, _) => {
                response.status(StatusCode::NOT_FOUND);
                Ok(response.body("<h1>404</h1><p>Not found!<p>\n".as_bytes().to_vec())?)
            }
        }
    });

    server.listen(host, port);
}
