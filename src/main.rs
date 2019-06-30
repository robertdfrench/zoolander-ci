mod router;

extern crate fastcgi;

use std::io::Write;
use std::net::TcpListener;

fn main() {
    let tcp = TcpListener::bind("127.0.0.1:9000").unwrap();

    fastcgi::run_tcp(|mut req| {
        let response = router::route_request();
        write!(&mut req.stdout(), "{}", response).unwrap_or(());
    }, &tcp)
}
