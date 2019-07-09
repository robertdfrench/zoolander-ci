/* A Dictionary is a Map<String,String> with the specific property that its
 * entries are sorted lexically by key. This is the behavior we want for HTTP
 * headers so that response documents are rendered the same way every time (as
 * this facilitates testing). */

use std::collections::HashMap;

pub struct Dictionary {
    entries: HashMap<String, String>
}

pub fn new() -> Dictionary {
    Dictionary{
        entries: HashMap::new()
    }
}

impl Dictionary {
    pub fn insert(&mut self, key: &str, value: &str) {
        self.entries.insert(String::from(key), String::from(value));
    }
    pub fn read(&self) -> Vec<(String, String)> {
        let mut contents = Vec::new();
        for (key, value) in &self.entries {
            contents.push((key.clone(), value.clone()));
        }
        contents.sort();
        contents
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers_is_a_sorted_iterator() {
        let mut r = new();
        r.insert("C", "3");
        r.insert("B", "2");
        r.insert("A", "1");

        fn to_string_tuples(str_tuples: Vec<(&str, &str)>) -> Vec<(String, String)> {
            str_tuples.iter().map(|&x| (String::from(x.0), String::from(x.1))).collect()
        }

        assert_eq!(
            r.read(),
            to_string_tuples(vec![("A", "1"), ("B", "2"), ("C", "3")])
        );
    }
}
