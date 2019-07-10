extern crate fastcgi;

use std::io::{Read,Write};
use std::net::TcpListener;

mod http_document;
mod push_event;
mod supervisor;

use push_event::PushEvent;

fn launch(content: &str) -> String {
    let parse: serde_json::Result<PushEvent> = serde_json::from_str(content);
    match parse {
        Err(e) => http_document::error(&format!("Could not parse payload. {}", e)),
        Ok(push_event) => match supervisor::spawn_job(&push_event.after) {
            Ok(_) => http_document::okay("Launched supervisor"),
            Err(_) => http_document::error("Could not launch supervisor")
        }
    }
}


fn read_log(uri: &str) -> String {
    let mut components = uri.split("/").collect::<Vec<&str>>();
    match components.pop() {
        None => http_document::not_found("No such job."),
        Some(commit) => match supervisor::read_job_log(&commit) {
            Ok(v) => http_document::okay(&v),
            Err(_) => http_document::not_found("No such job.")
        }
    }
}

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

fn serve_fcgi(socket: TcpListener) {
    fastcgi::run_tcp(move |mut req| {
        let response = match req.method().as_str() {
            "GET" => read_log(&req.uri()),
            "POST" => launch(&req.content()),
            _ => http_document::method_not_allowed("Can't route this request")
        };
        req.respond_with(&response);
    }, &socket)
}

fn main() {
    let address = "127.0.0.1:9000";
    match TcpListener::bind(address) {
        Ok(socket) => serve_fcgi(socket),
        Err(e) => panic!("Could not bind to {}. {}", address, e)
    };
}


#[cfg(test)]
mod integration {
    use super::*;

    use std::fs::File;
    use std::fs;

    #[test]
    fn can_read_log() {
        fs::create_dir_all("jobs/ab").unwrap();
        let mut file = File::create("jobs/ab/c123").unwrap();
        file.write_all(b"Hello").unwrap();

        let response = read_log("/jobs/abc123");
        assert_eq!(response, "Content-Type: text/plain\nStatus: 200 OK\n\nHello");
    }

    #[test]
    fn can_read_empty_log() {
        let response = read_log("/jobs/abc124");
        assert_eq!(response, "Content-Type: text/plain\nStatus: 404 Not Found\n\nNo such job.");
    }
}
