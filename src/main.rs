extern crate hyper;
mod validation_service;

use hyper::{Server};
use hyper::service::service_fn;
use hyper::rt::Future;

fn main() {
    // Socket address
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(|| service_fn(validation_service::service)).map_err(|e| eprint!("server error: {}", e));

    hyper::rt::run(server);
}