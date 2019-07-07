pub fn pathify(uuid: &str) -> String {
    let mut path = String::from(uuid);
    if path.len() > 2 { path.insert(2, '/') };
    String::from("jobs/") + &path
}

pub fn basename(path: &str) -> String {
    let mut components = path.split("/").collect::<Vec<&str>>();
    match components.pop() {
        Some(v) => v,
        None => ""
    }.to_string()
}

pub fn parent(uuid: &str) -> String {
    match uuid.len() {
        0 ... 2 => String::from("jobs/"),
        _ => format!("jobs/{}", &uuid[..2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        assert_eq!(pathify("abc"), "jobs/ab/c");
    }

    #[test]
    fn short() {
        assert_eq!(pathify("ab"), "jobs/ab");
    }
}
