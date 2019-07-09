extern crate fastcgi;

use std::fs;
use std::io::{Read,Write};
use std::net::TcpListener;
use std::process::{Command,Child};

mod pathify;
mod http_document;
mod push_event;

use pathify::*;
use push_event::PushEvent;

fn launch(content: &str) -> String {
    let parse: serde_json::Result<PushEvent> = serde_json::from_str(content);
    match parse {
        Err(e) => http_document::okay(&format!("Could not parse payload. {}", e)),
        Ok(push_event) => spawn(&push_event.after)
    }
}

fn bash(command: &str) -> std::io::Result<Child> {
    Command::new("bash").arg("-c").arg(command).spawn()
}

fn spawn(commit: &str) -> String {

    fn with_directory(commit: &str) -> String {
        let command = format!("bash supervisor.sh {} > {} 2>&1", commit, pathify(commit));
        match bash(&command) {
            Ok(_) => http_document::okay("Launched supervisor"),
            Err(_) => http_document::okay("Could not launch supervisor")
        }
    }

    match fs::create_dir_all(parent(commit)) {
        Ok(_) => with_directory(commit),
        Err(_) => http_document::okay("Could not create working directory")
    }
}

fn read_log(uri: &str) -> String {
    let path = basename(uri);
    let job_output = fs::read_to_string(&pathify(&path.to_string()));
    match job_output {
        Ok(v) => http_document::okay(&v),
        Err(_) => http_document::not_found("No such job.")
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

    #[test]
    fn spawns_make() {
        assert_eq!(spawn("112233"), "Content-Type: text/plain\nStatus: 200 OK\n\nLaunched supervisor");
    }
}
