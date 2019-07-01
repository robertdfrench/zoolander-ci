use std::collections::HashMap;

pub struct Zoolander {
    routes: HashMap<String, fn() -> String>
}

impl Zoolander {
    pub fn new() -> Zoolander {
        Zoolander { routes: HashMap::new() }
    }

    pub fn handle(self: &Self, path: &str) -> String {
        self.routes.get(path).unwrap()()
    }

    pub fn route(self: &mut Self, path: &str, handler: fn() -> String) {
        self.routes.insert(path.to_string(), handler);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_route() {
        let mut app = Zoolander::new();
        fn post() -> String { "poop".to_string() }
        app.route("POST /", post);

        assert_eq!(app.handle("POST /"), "poop");
    }
}
