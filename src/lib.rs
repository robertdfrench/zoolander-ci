mod interior;
mod perimeter;

use interior::application::Zoolander;
use interior::zone::name;
use interior::http_document::text_plain;
use interior::pathify::pathify;
use perimeter::Application;
pub use perimeter::serve_fcgi;
pub use interior::http_document;
use std::fs;

impl Application for Zoolander {
    fn handle(self: &Self) -> String {
        self.handle("GET /")
    }
}

pub fn greet() -> String {
    text_plain(name())
}

pub fn read_log(d: http_document::HttpDocument) -> String {
    let uri = d.read_header(&"REQUEST_URI".to_string());
    let path = uri.split("/").collect::<Vec<&str>>()
        .pop().unwrap();
    let job_output = fs::read_to_string("jobs/".to_owned() + &pathify(path.to_string()));
    match job_output {
        Ok(v) => text_plain(&v),
        Err(_) => "Status: 404\nContent-Type: text/plain\n\nNo such job.".to_string()
    }
}

pub fn new() -> Zoolander {
    Zoolander::new()
}
