// Influenced by:
// https://github.com/hyperium/hyper/tree/master/examples
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper::{Method, Request};
use hyper_tls::HttpsConnector;
use serde_json::Error;

// Sorry about not using snake case names
// This causes a lot of warnings.  The problem is that these need to be the
// same as the column names of the table in the database
//////////////////////////////////////////////////////////////////////////////////
// I tried using some serde directives that enable lower casing, but it
// doesn't work
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
struct Body
{
    User_Authentication_Key: Option<String>,
    Speech_URL: Option<String>,
    Translation_URL: Option<String>,
    Success: bool,
    Message: String
}

// NOTE: YOU MUST CHECK THE OUTPUT
// These tests unfortunately will always pass!
/////////////////////////////////////////////////
// Because the asserts are imbedded in a 'futures' call
// the test won't panic when if the assert fails
// I tried to have the future call return back data
// so I could assert the values outside (after) the
// futures call but couldn't figure that out
#[test]
fn test_auth_api_positive()
{
    let expected_response_body = Body
        {
            User_Authentication_Key: Some("authkey123".to_string()),
            Speech_URL: Some("wss://asr.acme.com:12345".to_string()),
            Translation_URL: Some("mt1.lovoco.co".to_string()),
            Success: true,
            Message: "".to_string()
        };

    let json= r#"{"username": "joeblow","password":"password"}"#;

    let url = "http://127.0.0.1:1337/api/LoginAPI/WinAppAuthAPI".parse().unwrap();
    //rt::run(fetch_url(url, json,expected_response_body));

    rt::run(fetch_url(url, json,expected_response_body));
}

#[test]
// I am not handling cases yet where the lookup doesn't find anything.
// A panic! is issued instead
fn test_auth_api_invalid_user()
{
    let expected_response_body = Body
        {
            User_Authentication_Key: Some("authkey123".to_string()),
            Speech_URL: Some("wss://asr.acme.com:12345".to_string()),
            Translation_URL: Some("mt1.lovoco.co".to_string()),
            Success: true,
            Message: "".to_string()
        };

    let json= r#"{"username": "XXuserXdoesXnotXExist","password":"XXXX"}"#;

    let url = "http://127.0.0.1:1337/api/LoginAPI/WinAppAuthAPI".parse().unwrap();
    rt::run(fetch_url(url, json,expected_response_body));
}

#[test]
// I am not handling cases where the user exists but the password is incorrect.
// A panic! is issued instead
fn test_auth_api_invalid_password()
{
    let expected_response_body = Body
        {
            User_Authentication_Key: Some("authkey123".to_string()),
            Speech_URL: Some("wss://asr.acme.com:12345".to_string()),
            Translation_URL: Some("mt1.lovoco.co".to_string()),
            Success: true,
            Message: "".to_string()
        };

    let json= r#"{"username": "joeblow","password":"XXXX"}"#;

    let url = "http://127.0.0.1:1337/api/LoginAPI/WinAppAuthAPI".parse().unwrap();
    rt::run(fetch_url(url, json,expected_response_body));
}

fn fetch_url(url: hyper::Uri, json: &'static str, expected_res_body: Body) -> impl Future<Item=(), Error=()>
{
    let mut req = Request::new(hyper::body::Body::from(json));

    *req.method_mut() = Method::POST;
    *req.uri_mut() = url.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        hyper::header::HeaderValue::from_static("application/json")
    );

    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);

    client
        // Fetch the url...
        //.get(url)
        .request(req)
        // And then, if we get a response back...
        .and_then(move | res|
        {
            // The body is a stream, and for_each returns a new Future
            // when the stream is finished, and calls the closure on
            // each chunk of the body...
            res.into_body().for_each(move |chunk|
            {
                let vec = chunk.to_vec();
                let response_json = String::from_utf8(vec).unwrap();
                //println!("response_json = {}", response_json);
                let res_body = serialize_response(&response_json).unwrap();

                assert_eq!(res_body.Success, expected_res_body.Success);
                assert_eq!(res_body.User_Authentication_Key, expected_res_body.User_Authentication_Key);
                assert_eq!(res_body.Speech_URL, expected_res_body.Speech_URL);
                assert_eq!(res_body.Translation_URL, expected_res_body.Translation_URL);
                assert_eq!(res_body.Message, expected_res_body.Message);

                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        // If all good, just tell the user...
        .map(|_| {
            println!("\n\nDone.");
        })
        // If there was an error, let the user know...
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}

fn serialize_response(json: &str) -> Result<Body, Error>
{
    let b: Body = serde_json::from_str(json)?;

    Ok(b)
}

