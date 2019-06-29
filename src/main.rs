mod zone;

extern crate fastcgi;

use std::io::Write;
use std::net::TcpListener;

fn main() {
    let tcp = TcpListener::bind("127.0.0.1:9000").unwrap();

    fastcgi::run_tcp(|mut req| {
        write!(&mut req.stdout(), "Content-Type: text/plain\n\n{}", zone::name())
            .unwrap_or(());
    }, &tcp)
}
