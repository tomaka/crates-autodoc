#![feature(globs)]

extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::response::modifiers::{Status, Body};

pub fn build_website() -> Box<iron::Handler + Send + Sync> {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::new().set(Status(status::Ok)).set(Body("Hello World!")))
    }

    box hello_world as Box<iron::Handler + Send + Sync>
}
