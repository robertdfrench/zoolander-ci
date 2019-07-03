extern crate fastcgi;

use std::fs;
use std::io::Write;
use std::net::TcpListener;

mod zone;
mod pathify;
mod http_document;

use zone::name;
use pathify::pathify;

pub fn greet() -> String {
    http_document::text_plain(name())
}

pub fn read_log(uri: String) -> String {
    let path = uri.split("/").collect::<Vec<&str>>()
        .pop().unwrap();
    let job_output = fs::read_to_string("jobs/".to_owned() + &pathify(path.to_string()));
    match job_output {
        Ok(v) => http_document::text_plain(&v),
        Err(_) => "Status: 404\nContent-Type: text/plain\n\nNo such job.".to_string()
    }
}

pub fn invalid_request() -> String {
    http_document::text_plain("Wtf son")
}

fn main() {
    let socket = TcpListener::bind("127.0.0.1:9000").unwrap();
    fastcgi::run_tcp(move |mut req| {
        let method = req.param("REQUEST_METHOD").unwrap();
        let uri = req.param("REQUEST_URI").unwrap();
        let response = match method.as_str() {
            "GET" => read_log(uri),
            "POST" => greet(),
            _ => invalid_request()
        };
        write!(&mut req.stdout(), "{}", response).unwrap_or(());
    }, &socket)
}


#[cfg(test)]
mod integration {
    use super::*;

    use std::fs::File;
    use std::fs;

    #[test]
    fn can_greet() {
        assert_eq!(greet(), "Content-Type: text/plain\n\nglobal".to_string())
    }

    #[test]
    fn can_read_log() {
        fs::create_dir_all("jobs/ab").unwrap();
        let mut file = File::create("jobs/ab/c123").unwrap();
        file.write_all(b"Hello").unwrap();

        let response = read_log("/jobs/abc123".to_string());
        assert_eq!(response, "Content-Type: text/plain\n\nHello");
    }

    #[test]
    fn can_read_empty_log() {
        let response = read_log("/jobs/abc124".to_string());
        assert_eq!(response, "Status: 404\nContent-Type: text/plain\n\nNo such job.");
    }
}
