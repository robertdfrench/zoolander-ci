use std::string::String;

pub fn text_plain(body: &str) -> String {
    let header = "Content-Type: text/plain";
    [header, body].join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_plain() {
        assert_eq!(text_plain("hello"), "Content-Type: text/plain\n\nhello");
    }
}
