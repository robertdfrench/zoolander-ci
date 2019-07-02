use zoolander_ci::*;
use std::fs::File;
use std::io::prelude::*;
use std::fs;

#[test]
fn can_greet() {
    assert_eq!(greet(), "Content-Type: text/plain\n\nglobal".to_string())
}

#[test]
fn can_read_log() {
    let mut req = http_document::new();
    req.write_header("REQUEST_URI".to_string(), "/jobs/abc123".to_string());

    fs::create_dir_all("jobs/ab").unwrap();
    let mut file = File::create("jobs/ab/c123").unwrap();
    file.write_all(b"Hello");

    assert_eq!(read_log(req), "Content-Type: text/plain\n\nHello");
}

#[test]
fn can_read_empty_log() {
    let mut req = http_document::new();
    req.write_header("REQUEST_URI".to_string(), "/jobs/abc124".to_string());

    assert_eq!(read_log(req), "Status: 404\nContent-Type: text/plain\n\nNo such job.");
}
