use std::string::String;
use std::collections::HashMap;

pub fn text_plain(body: &str) -> String {
    let mut doc = new();
    doc.write_header("Content-Type".to_string(), "text/plain".to_string());
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
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for (header, value) in &self.headers {
            s.push_str(format!("{}: {}\n", header, value).as_str());
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
}
