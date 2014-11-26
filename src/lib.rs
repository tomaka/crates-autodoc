#![feature(if_let)]
#![feature(globs)]
#![feature(slicing_syntax)]

extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::response::modifiers::{Status, Body};

use std::io::MemReader;

mod worker;

pub struct Config {
    pub s3_bucket: String,
}

pub fn build_website(config: Config) -> Box<iron::Handler + Send + Sync> {
    let mut router = router::Router::new();

    //router.get("/", homepage);
    router.get("/crates/:crate", crate_handler);

    box router as Box<iron::Handler + Send + Sync>
}

fn crate_handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.find::<router::Router, router::Params>().unwrap().find("crate").unwrap();

    Ok(Response {
        status: Some(status::Status::Ok),
        body: Some(box MemReader::new(query.as_bytes().to_vec()) as Box<Reader + Send>),
        .. Response::new()
    })
}
