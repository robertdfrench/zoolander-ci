mod interior;
mod perimeter;

use interior::application::Zoolander;
use interior::zone::name;
use interior::http_document::text_plain;
use perimeter::Application;
pub use perimeter::serve_fcgi;

impl Application for Zoolander {
    fn handle(self: &Self) -> String {
        self.handle()
    }
}

pub fn greet() -> String {
    text_plain(name())
}

pub fn new() -> Zoolander {
    Zoolander::new()
}
