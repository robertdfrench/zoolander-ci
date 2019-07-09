mod dictionary;

use dictionary::Dictionary;

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
    headers: Dictionary,
    body: String
}

impl HttpDocument {
    fn write_header(self: &mut Self, k: &str, v: &str) {
        self.headers.insert(k, v);
    }
    fn append_body(&mut self, content: &str) {
        self.body.push_str(content);
    }
    fn to_string(&self) -> String {
        let mut s = String::new();
        for (header, value) in self.headers.read() {
            s.push_str(format!("{}: {}\n", header, value).as_str())
        }
        s.push_str("\n");
        s.push_str(self.body.as_str());
        s
    }
}

fn new() -> HttpDocument {
    HttpDocument{
        headers: dictionary::new(),
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
}
