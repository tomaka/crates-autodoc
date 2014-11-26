#![feature(globs)]
#![feature(slicing_syntax)]

extern crate "crates-autodoc" as autodoc;
extern crate iron;

use std::io::net::ip::IpAddr::Ipv4Addr;
use iron::Iron;

fn main() {
    let port = std::os::getenv("PORT").map(|n| from_str(n[]).unwrap()).unwrap_or(8000);
    Iron::new(autodoc::build_website()).listen((Ipv4Addr(0, 0, 0, 0), port)).unwrap();
    println!("Listening on {}", port);
}
