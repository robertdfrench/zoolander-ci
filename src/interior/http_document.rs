use std::string::String;
use std::collections::HashMap;

pub fn text_plain(body: &str) -> String {
    let header = "Content-Type: text/plain";
    [header, body].join("\n\n")
}

pub struct HttpDocument {
    headers: HashMap<String, String>,
    body: String
}

impl HttpDocument {
    pub fn write_header(self: &mut Self, k: String, v: String) {
        self.headers.insert(k, v);
    }
    pub fn read_header(self: &Self, k: &String) -> String {
        self.headers.get(k).unwrap().to_string()
    }
}

pub fn new() -> HttpDocument {
    let mut h = HttpDocument{
        headers: HashMap::new(),
        body: "".to_string()
    };
    h.headers.insert("Content-Type".to_string(), "text/plain".to_string());
    h
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
        let r = new();
        assert!(r.headers.contains_key("Content-Type"))
    }
}
