mod application;
mod http_document;
mod zone;

extern crate fastcgi;

use std::io::Write;
use std::net::TcpListener;
use application::Zoolander;

fn fcgi_start(app: Zoolander, address: &str) {
    let socket = TcpListener::bind(address).unwrap();

    fastcgi::run_tcp(move |mut req| {
        let response = app.handle();
        write!(&mut req.stdout(), "{}", response).unwrap_or(());
    }, &socket)
}

fn main() {
    let mut app = Zoolander::new();

    app.route("GET /", || {
        http_document::text_plain(zone::name())
    });

    fcgi_start(app, "127.0.0.1:9000")
}
