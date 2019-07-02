use serde::{Deserialize};

#[derive(Deserialize)]
pub struct PushEvent {
    r#ref: String,
    after: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_a_push_event() {
        let input = r#"{
            "ref": "abc123",
            "after": "321cba",
            "other": {
                "ignore": true
            }
        }"#;
        let e: PushEvent = serde_json::from_str(input).unwrap();
        assert_eq!(e.r#ref, "abc123");
        assert_eq!(e.r#after, "321cba");
    }
}
