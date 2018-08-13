extern crate hyper;
extern crate futures;

use self::futures::future;
use hyper::rt::Future;
use hyper::{Body, Method, Request, Response, StatusCode};

type BoxFut = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

pub fn service(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        },
        (&Method::GET, "/validation") => {
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        },
        (&Method::POST, "/validation") => {
            *response.body_mut() = req.into_body();
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }


    Box::new(future::ok(response))
}