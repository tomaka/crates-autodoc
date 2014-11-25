#![feature(slicing_syntax)]

extern crate "crates-autodoc" as autodoc;
extern crate iron;

use std::io::net::ip::IpAddr::Ipv4Addr;

use iron::Iron;
use iron::response::modifiers::Body;

fn main() {
    fn hello_world(req: &mut iron::Request) -> iron::IronResult<iron::Response> {
        Ok(iron::Response {
            status: Some(iron::status::Ok),
            .. iron::Response::new()
        })
    }

    let port = std::os::getenv("PORT").map(|n| from_str(n[]).unwrap()).unwrap_or(8000);
    Iron::new(hello_world).listen((Ipv4Addr(127, 0, 0, 1), port));
}
