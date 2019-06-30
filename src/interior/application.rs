use std::collections::HashMap;

pub struct Zoolander {
    routes: HashMap<String, fn() -> String>
}

impl Zoolander {
    pub fn new() -> Zoolander {
        Zoolander { routes: HashMap::new() }
    }

    pub fn handle(self: &Self) -> String {
        self.routes.get("GET /").unwrap()()
    }

    pub fn route(self: &mut Self, path: &str, handler: fn() -> String) {
        self.routes.insert(path.to_string(), handler);
    }
}
