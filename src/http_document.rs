use std::string::String;
use std::collections::HashMap;

pub fn okay(body: &str) -> String {
    plaintext_response("200 OK", body)
}

pub fn not_found(body: &str) -> String {
    plaintext_response("404 Not Found", body)
}

pub fn method_not_allowed(body: &str) -> String {
    plaintext_response("405 Method Not Allowed", body)
}

pub fn error(body: &str) -> String {
    plaintext_response("500 Internal Server Error", body)
}

fn plaintext_response(status: &str, body: &str) -> String {
    let mut doc = new();
    doc.write_header("Content-Type", "text/plain");
    doc.write_header("Status", status);
    doc.append_body(body);
    doc.to_string()
}

struct HttpDocument {
    headers: HashMap<String, String>,
    body: String
}

impl HttpDocument {
    fn write_header(self: &mut Self, k: &str, v: &str) {
        self.headers.insert(String::from(k), String::from(v));
    }
    fn append_body(&mut self, content: &str) {
        self.body.push_str(content);
    }
    fn sorted_headers(&self) -> Vec<&str> {
        let mut h = Vec::new();
        for header in self.headers.keys() {
            h.push(header.as_str())
        }
        h.sort();
        h
    }
    fn to_string(&self) -> String {
        let mut s = String::new();
        for header in self.sorted_headers() {
            match self.headers.get(header) {
                Some(value) => s.push_str(format!("{}: {}\n", header, value).as_str()),
                None => unreachable!()
            };
        }
        s.push_str("\n");
        s.push_str(self.body.as_str());
        s
    }
}

fn new() -> HttpDocument {
    HttpDocument{
        headers: HashMap::new(),
        body: String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_okay() {
        assert_eq!(okay("hello"), "Content-Type: text/plain\nStatus: 200 OK\n\nhello");
    }

    #[test]
    fn new_has_content_type() {
        let mut r = new();
        r.write_header("Content-Type", "text/plain");
        assert!(r.headers.contains_key("Content-Type"))
    }

    #[test]
    fn sorted_headers() {
        let mut r = new();
        r.write_header("D", "4");
        r.write_header("C", "3");
        r.write_header("B", "2");
        r.write_header("A", "1");
        assert_eq!(r.to_string(), "A: 1\nB: 2\nC: 3\nD: 4\n\n");
    }

    #[test]
    fn headers_is_a_sorted_iterator() {
        let mut r = new();
        r.write_header("D", "4");
        r.write_header("C", "3");
        r.write_header("B", "2");
        r.write_header("A", "1");
        assert_eq!(r.sorted_headers(), vec!["A", "B", "C", "D"]);
    }
}
