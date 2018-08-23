extern crate auth_server;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate unicase;

use futures::future::*;
use unicase::Ascii;
use std::str;

use self::auth_server::*;
use futures::{future, Future, Stream};
use hyper::{Body, Chunk, Client, Method, Request, Response, Server, StatusCode, header};
use hyper::client::HttpConnector;
use hyper::service::service_fn;
use hyper_tls::HttpsConnector;

#[test]
fn test_get_credentials()
{
    let expect_res = r#"{"User_Authentication_Key": "authkey123", "Speech_URL": "wss://asr.acme.com:12345", "Translation_URL":"mt1.lovoco.co", "Success":true, "Message":""}"#;
    let actual_res = get_credentials("joeblow", "password");

    println!("expect_res: {}", expect_res);
    println!("actual_res: {}", actual_res);
    assert!(expect_res == actual_res);
}

#[test]
fn test_serialize_request()
{
    let req_body = r#"{"username": "joeblow", "password": "password" }"#;

    let req_app_user: AppUserRequest = serialize_request(&req_body).unwrap();

    let username = &req_app_user.username.unwrap();
    let password = &req_app_user.password.unwrap();

    assert!(username == "joeblow");
    assert!(password == "password");
}

// Could not for the life of me figure out how to create a Hyper::Request<Body>
// for testing purposes.  I basically need to mock a request body to test
// This was badly needed to isolate errors.  Oh well.
//#[test]
//fn test_toupper()
//{
//
//
////    let b = Body
////        {
////            User_Authentication_Key: Some("f84089af-2dc4-4119-b671-e8e297b4dd34".to_string()),
////            Speech_URL: Some("wss://services.govivace.com:49153".to_string()),
////            Translation_URL: Some("mt.lovoco.co".to_string()),
////            Success: true,
////            Message: "".to_string()
////        };
//
//
////    let body =
////        {
////
////        }
//
//    //use http::{Request, Response};
//    let json= r#"{"UserName": "joeblow","Password":"password"}"#;
//    let mut req = hyper::Request::new(hyper::body::Body::from(json));
//    let url: hyper::Uri = "http://127.0.0.1:1337/api/LoginAPI/WinAppAuthAPI".parse().unwrap();
//
//    *req.method_mut() = Method::POST;
//    *req.uri_mut() = url.clone();
//    req.headers_mut().insert(
//        hyper::header::CONTENT_TYPE,
//        hyper::header::HeaderValue::from_static("application/json"));
//
//    let https = hyper_tls::HttpsConnector::new(4).expect("TLS initialization failed");
//    let client = Client::builder()
//        .build::<_, hyper::Body>(https);
//
//    let response_body = to_upper(req);
//
//    //response_body.map();
//    //response_body.
//    //let s:String = from(response_body());
//
////    let response = response_body.body().concat2().map_err(|_err| ()).map(|chunk|
////    {
////        let v = chunk.to_vec();
////        String::from_utf8_lossy(&v).to_string()
////    });
//
//
////    let blah1 = response_body.concat2()
////        .and_then(|body| {
////            let stringify = str::from_utf8(&body).unwrap();
////            println!("{}", stringify);
////            futures::future::ok(response.with_headers(headers))
////        }).boxed();
//
//    let mut fuck = "";
//    let response = response_body.concat2()
//        .and_then(|body|
//        {
//            fuck = str::from_utf8(&body).unwrap();
//            futures::future::ok(response.with_headers(headers))
//        });
//
//    println!("response: {}", response);
//
////    //let response = response_body.body().concat2().map_err(|_err| ()).map(|chunk|
////    let response = response_body.concat2().map_err(|_err| ()).map(|chunk|
////    {
////        let v = chunk.to_vec();
////        String::from_utf8_lossy(&v).to_string()
////    });
////
////    println!("response: {}", &response);
////    //let username = &response_body.username.unwrap();
////    //let password = &response_body.password.unwrap();
//
//    assert!(false);
//
//}
