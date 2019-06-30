use zoolander_ci::*;

#[test]
fn can_greet() {
    assert_eq!(greet(), "Content-Type: text/plain\n\nglobal".to_string())
}
