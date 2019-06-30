extern crate fastcgi;

use std::io::Write;
use std::net::TcpListener;

pub trait Application {
    fn handle(self: &Self) -> String;
}

pub fn serve_fcgi<A: 'static>(app: A, address: &str) 
    where A: Application + std::marker::Send + std::marker::Sync 
{
    let socket = TcpListener::bind(address).unwrap();

    fastcgi::run_tcp(move |mut req| {
        let response = app.handle();
        write!(&mut req.stdout(), "{}", response).unwrap_or(());
    }, &socket)
}
