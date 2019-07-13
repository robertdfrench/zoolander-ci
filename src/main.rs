mod http_document;
mod push_event;
mod supervisor;
mod fcgi;

fn launch(content: &str) -> String {
    match push_event::from_str(&content) {
        Err(e) => http_document::error(&format!("Could not parse payload. {}", e)),
        Ok(push_event) => match supervisor::spawn_job(&push_event.after) {
            Ok(_) => http_document::okay("Launched supervisor"),
            Err(_) => http_document::error("Could not launch supervisor")
        }
    }
}


fn read_log(uri: &str) -> String {
    let mut components = uri.split("/").collect::<Vec<&str>>();
    match components.pop() {
        None => http_document::bad_request("Read requests uris must contain a commit hash"),
        Some(commit) => match supervisor::read_job_log(&commit) {
            Ok(v) => http_document::okay(&v),
            Err(_) => http_document::not_found("No such job.")
        }
    }
}

fn main() {
    let address = "127.0.0.1:9000";
    let error = fcgi::start(address, |method, uri, content| {
        match method {
            "GET" => read_log(uri),
            "POST" => launch(content),
            _ => http_document::method_not_allowed("Can't route this request")
        }
    }); 
    panic!("Could not bind to {}. {}", address, error);
}


#[cfg(test)]
mod integration {
    use super::*;

    use std::fs::File;
    use std::fs;
    use std::io::Write;


    #[test]
    fn can_read_log() {
        fs::create_dir_all("jobs/ab").unwrap();
        let mut file = File::create("jobs/ab/c123").unwrap();
        file.write_all(b"Hello").unwrap();

        let response = read_log("/jobs/abc123");
        assert_eq!(response, "Content-Type: text/plain\nRefresh: 1\nStatus: 200 OK\n\nHello");
    }

    #[test]
    fn can_read_empty_log() {
        let response = read_log("/jobs/abc124");
        assert_eq!(response, "Content-Type: text/plain\nRefresh: 1\nStatus: 404 Not Found\n\nNo such job.");
    }
}
