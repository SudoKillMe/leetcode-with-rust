extern crate hyper;

use hyper::{ Body, Request, Response, Server };
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::service::service_fn;
use hyper::{ Method, StatusCode };
use futures::future;
use futures::Stream;
use hyper::Chunk;

type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

const PHRASE: &str = "Hello, world";

fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTING data to /echo")
        },
        (&Method::POST, "/") => {
            //
            let mapping = req
                .into_body()
                .map(|chunk| {
                    chunk.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Vec<u8>>()
                });

            // Use `Body::wrap_stream` to convert it to a `Body`...
            *response.body_mut() = Body::wrap_stream(mapping);
        },
        (&Method::POST, "/echo/reverse") => {
            let reversed = req
                .into_body()
                .concat2()
                .map(move |chunk| {
                    let body = chunk.iter()
                        .rev()
                        .cloned()
                        .collect::<Vec<u8>>();

                    *response.body_mut() = Body::from(body);
                    response
                });

            return Box::new(reversed);
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

fn hello_world(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(PHRASE))
}

fn main() {
    let addr = ([127,0,0,1], 3000).into();

//    let new_svc = || {
//        service_fn_ok(hello_world)
//    };

    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
