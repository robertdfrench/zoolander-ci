mod zone;
mod http_document;

pub fn route_request() -> String {
    return http_document::text_plain(zone::name());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_request() {
        assert_eq!(route_request(), "Content-Type: text/plain\n\nglobal");
    }
}
