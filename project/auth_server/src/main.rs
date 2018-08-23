/////////////////////////////////////////////////////////////////
// Steve Braich sbraich@pdx.edu
// Authorization Server - Web API
/////////////////////////////////////////////////////////////////
// Heavily influence by:
// (HTTP POST) https://github.com/actix/examples/tree/master/basics
// (HTTP POST) https://github.com/hyperium/hyper/tree/master/examples
// (Diesel - SQL)  https://github.com/diesel-rs/diesel/tree/master/examples/sqlite

#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate serde_json;
extern crate diesel;

extern crate auth_server;

use self::auth_server::*;
use futures::{future, Future, Stream};
use hyper::{Body, Chunk, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;

#[allow(unused, deprecated)]
use std::ascii::AsciiExt;

static NOTFOUND: &[u8] = b"Not Found";

fn response_examples(req: Request<Body>)
                     -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>
{
    match (req.method(), req.uri().path())
    {
        (&Method::POST, "/api/LoginAPI/WinAppAuthAPI") =>
        {
            let body = post_auth_request_handler(req);

            Box::new(future::ok(Response::new(body)))
        },

        (&Method::POST, "/api/upper") =>
        {
            // A web api to run against. Uppercases the body and returns it back.
            let body = post_upper_request_handler(req);
            Box::new(future::ok(Response::new(body)))
        },

        _ =>
        {
            // Return 404 not found response.
            let body = Body::from(NOTFOUND);
            Box::new(future::ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body)
                .unwrap()))
        }
    }
}

/// This method processes authenticantion POST requests
pub fn post_auth_request_handler(req: Request<Body>) -> Body
{
    let body = Body::wrap_stream(req.into_body().map(|chunk|
    {
        // Get the request body
        let vec = chunk.to_vec();
        let req_body = String::from_utf8(vec).unwrap();

        let req_app_user: AppUserRequest = serialize_request(&req_body).unwrap();

        let username = &req_app_user.username.unwrap();
        let password = &req_app_user.password.unwrap();

        let res = get_credentials(username, password);

        Chunk::from(res)
    }));

    body
}

// This is really just a test function
// It is used to isolate issues with web api calls.
// This one doesn't hit the database so we can just test HTTP POST
// Left in here because debugging a web server is not easy.
pub fn post_upper_request_handler(req: Request<Body>) -> Body
{
    // A web api to run against. Uppercases the body and returns it back.
    let body = Body::wrap_stream(req.into_body().map(|chunk|
        {
            // uppercase the letters
            // Original
            let upper = chunk.iter().map(|byte| byte.to_ascii_uppercase()).collect::<Vec<u8>>();
            Chunk::from(upper)
        }));

    body
}


fn main()
{
    pretty_env_logger::init();

    let addr = "127.0.0.1:1337".parse().unwrap();

    hyper::rt::run(future::lazy(move ||
    {
        // Share a `Client` with all `Service`s
        //let client = Client::new();

        let new_service = move ||
        {
            // Move a clone of `client` into the `service_fn`.
            //let client = client.clone();
            service_fn(move | req|
            {
                //response_examples(req, &client)
                response_examples(req)
            })
        };

        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(| e| eprintln!("server error: {}", e));

        println!("Listening on http://{}", addr);

        server
    }));
}
