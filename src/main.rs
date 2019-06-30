mod zone;
mod http_document;

extern crate fastcgi;

use std::io::Write;
use std::net::TcpListener;

fn main() {
    let tcp = TcpListener::bind("127.0.0.1:9000").unwrap();

    fastcgi::run_tcp(|mut req| {
        let response = http_document::text_plain(zone::name());
        write!(&mut req.stdout(), "{}", response)
            .unwrap_or(());
    }, &tcp)
}
