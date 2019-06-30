use zoolander_ci::*;

fn main() {
    let mut app = new();

    app.route("GET /", greet);

    serve_fcgi(app, "127.0.0.1:9000")
}
