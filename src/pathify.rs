pub fn pathify(uuid: &String) -> String {
    let mut path = uuid.clone();
    if path.len() > 2 { path.insert(2, '/') };
    "jobs/".to_string() + &path
}

pub fn basename(path: String) -> String {
    let mut components = path.split("/").collect::<Vec<&str>>();
    match components.pop() {
        Some(v) => v,
        None => ""
    }.to_string()
}

pub fn parent(uuid: &String) -> String {
    match uuid.len() {
        0 ... 2 => "jobs/".to_string(),
        _ => format!("jobs/{}", &uuid[..2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        assert_eq!(pathify(&"abc".to_string()), "jobs/ab/c");
    }

    #[test]
    fn short() {
        assert_eq!(pathify(&"ab".to_string()), "jobs/ab");
    }
}
