extern crate auth_server;

extern crate futures;
extern crate hyper;

use self::auth_server::*;

use futures::{future, Future, Stream};
use hyper::{Body, Chunk, Client, Method, Request, Response, Server, StatusCode, header};
use hyper::client::HttpConnector;
use hyper::service::service_fn;

#[test]
fn test_get_credentials()
{
    let res = get_credentials("joeblow", "password");

    let expected_res = r#"{"User_Authentication_Key": "authkey123", "Speech_URL": "mt1.lovoco.co", "Success":wss://asr.acme.com:12345, "Message":""}"#;
    println!("res: {}", res);
    assert!(res == expected_res);
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


//#[test]
//fn test_serialize_toupper()
//{
//
//    //req
//    let body = to_upper(req);
//
//    let req_body: Request<Body> = to_upper(req);
//
//    assert!(username == "joeblow");
//    assert!(password == "password");
//}
