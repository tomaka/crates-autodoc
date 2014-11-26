#![feature(if_let)]
#![feature(globs)]
#![feature(slicing_syntax)]

extern crate cargo;
extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::response::modifiers::{Status, Body};

use std::io::MemReader;
use std::sync::Arc;

mod worker;

pub struct Config {
    pub s3_bucket: String,
}

pub fn build_website(config: Config) -> Box<iron::Handler + Send + Sync> {
    let worker = Arc::new(worker::Worker::new(Path::new("tmp")));

    let mut router = router::Router::new();
    //router.get("/", homepage);
    router.get("/crates/:crate", crate_handler);

    let mut worker_chain = iron::middleware::ChainBuilder::new(router);
    worker_chain.link_before(BeforeWorker { worker: worker, });

    box worker_chain as Box<iron::Handler + Send + Sync>
}

struct BeforeWorker {
    worker: Arc<worker::Worker>,
}

struct HackyKeyTypeCauseIDontUnderstandHowThatExperimentalThingIsSupposedToWorkAndTheresNoDocAtAll;
type HackyThing = HackyKeyTypeCauseIDontUnderstandHowThatExperimentalThingIsSupposedToWorkAndTheresNoDocAtAll;
impl iron::typemap::Assoc<Arc<worker::Worker>> for HackyKeyTypeCauseIDontUnderstandHowThatExperimentalThingIsSupposedToWorkAndTheresNoDocAtAll {}

impl iron::middleware::BeforeMiddleware for BeforeWorker {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<HackyThing, _>(self.worker.clone());
        Ok(())
    }
}

fn crate_handler(req: &mut Request) -> IronResult<Response> {
    let ref crate_name = req.extensions.find::<router::Router, router::Params>().unwrap().find("crate").unwrap();

    let worker = req.extensions.get::<HackyThing, Arc<worker::Worker>>().unwrap();

    worker.submit(*crate_name);

    Ok(Response {
        status: Some(status::Status::Ok),
        body: Some(box MemReader::new(crate_name.as_bytes().to_vec()) as Box<Reader + Send>),
        .. Response::new()
    })
}
