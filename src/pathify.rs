pub fn pathify(uuid: String) -> String {
    let mut path = uuid.clone();
    if path.len() > 2 { path.insert(2, '/') };
    path
}

pub fn basename(path: String) -> String {
    let mut components = path.split("/").collect::<Vec<&str>>();
    match components.pop() {
        Some(v) => v,
        None => ""
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        assert_eq!(pathify("abc".to_string()), "ab/c");
    }

    #[test]
    fn short() {
        assert_eq!(pathify("ab".to_string()), "ab");
    }
}
