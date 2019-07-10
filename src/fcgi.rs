extern crate fastcgi;

use std::io::{Read,Write};
use std::net::TcpListener;

trait FCGIHelper {
    fn method(&self) -> String;
    fn uri(&self) -> String;
    fn content(&mut self) -> String;
    fn respond_with(&mut self, response: &str);
}

impl FCGIHelper for fastcgi::Request {
    fn method(&self) -> String {
        self.param("REQUEST_METHOD").unwrap_or("GET".to_string())
    }
    fn uri(&self) -> String {
        self.param("REQUEST_URI").unwrap_or("/".to_string())
    }
    fn content(&mut self) -> String {
        let mut b = String::new();
        match self.stdin().read_to_string(&mut b) {
            Ok(_) => b,
            Err(_) => "".to_string()
        }
    }
    fn respond_with(&mut self, response: &str) {
        match write!(&mut self.stdout(), "{}", response) {
            Ok(_) => (),
            Err(e) => println!("Failed to return a response. {}", e)
        };
    }
}

pub fn start<F>(address: &str, router: F) -> std::io::Error where
        F: Fn(&str, &str, &str) -> String  + Send + Sync + 'static {
    let handler = move |mut req: fastcgi::Request| {
        let response = router(&req.method(), &req.uri(), &req.content());
        req.respond_with(&response);
    };
    match TcpListener::bind(address) {
        Ok(socket) => {
            fastcgi::run_tcp(handler, &socket);
            unreachable!()
        },
        Err(e) => e
    }
}
