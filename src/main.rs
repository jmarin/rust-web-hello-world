#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, Nickel};

fn main() {
    let mut server = Nickel::new();
    server.get("**", middleware!("Hola Mundo"));
    server.listen("0.0.0.0:6767");
}
