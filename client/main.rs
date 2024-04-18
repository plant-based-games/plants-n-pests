#![forbid(unsafe_code)]
mod plugins;

use plugins::client;

fn main() {
    let mut app = client::create_app();
    app.run();
}
