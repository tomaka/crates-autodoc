#![feature(globs)]
#![feature(slicing_syntax)]

extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::response::modifiers::{Status, Body};

use std::io::net::ip::IpAddr::Ipv4Addr;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::new().set(Status(status::Ok)).set(Body("Hello World!")))
    }

    let port = std::os::getenv("PORT").map(|n| from_str(n[]).unwrap()).unwrap_or(8000);
    Iron::new(hello_world).listen((Ipv4Addr(0, 0, 0, 0), port)).unwrap();
    println!("Listening on {}", port);
}
