use std::string::String;
use std::collections::HashMap;

pub fn text_plain(body: &str) -> String {
    let mut doc = new();
    doc.write_header("Content-Type".to_string(), "text/plain".to_string());
    doc.append_body(body);
    doc.to_string()
}

pub fn not_found(body: &str) -> String {
    let mut doc = new();
    doc.write_header("Content-Type".to_string(), "text/plain".to_string());
    doc.write_header("Status".to_string(), "404 Not Found".to_string());
    doc.append_body(body);
    doc.to_string()
}

pub fn method_not_allowed(body: &str) -> String {
    let mut doc = new();
    doc.write_header("Content-Type".to_string(), "text/plain".to_string());
    doc.write_header("Status".to_string(), "405 Method Not Allowed".to_string());
    doc.append_body(body);
    doc.to_string()
}

pub struct HttpDocument {
    headers: HashMap<String, String>,
    body: String
}

impl HttpDocument {
    pub fn write_header(self: &mut Self, k: String, v: String) {
        self.headers.insert(k, v);
    }
    pub fn append_body(&mut self, content: &str) {
        self.body.push_str(content);
    }
    pub fn sorted_headers(&self) -> Vec<&str> {
        let mut h = Vec::new();
        for header in self.headers.keys() {
            h.push(header.as_str())
        }
        h.sort();
        h
    }
    pub fn to_string(&self) -> String {
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

pub fn new() -> HttpDocument {
    HttpDocument{
        headers: HashMap::new(),
        body: String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_text_plain() {
        assert_eq!(text_plain("hello"), "Content-Type: text/plain\n\nhello");
    }

    #[test]
    fn new_has_content_type() {
        let mut r = new();
        r.write_header("Content-Type".to_string(), "text/plain".to_string());
        assert!(r.headers.contains_key("Content-Type"))
    }

    #[test]
    fn sorted_headers() {
        let mut r = new();
        r.write_header("D".to_string(), "4".to_string());
        r.write_header("C".to_string(), "3".to_string());
        r.write_header("B".to_string(), "2".to_string());
        r.write_header("A".to_string(), "1".to_string());
        assert_eq!(r.to_string(), "A: 1\nB: 2\nC: 3\nD: 4\n\n");
    }

    #[test]
    fn headers_is_a_sorted_iterator() {
        let mut r = new();
        r.write_header("D".to_string(), "4".to_string());
        r.write_header("C".to_string(), "3".to_string());
        r.write_header("B".to_string(), "2".to_string());
        r.write_header("A".to_string(), "1".to_string());
        assert_eq!(r.sorted_headers(), vec!["A", "B", "C", "D"]);
    }
}
