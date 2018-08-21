extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde_json::Error;

use std::io::{self, Write};

use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper::{Method, Request};
//use hyper::header::{Connection, Headers, UserAgent};

use hyper_tls::HttpsConnector;


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

#[test]
fn test_hyper_client_post_http()
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
    rt::run(fetch_url(url, json,expected_response_body));
}


#[test]
fn test_hyper_client_post_ssl()
{
    let b = Body
    {
        User_Authentication_Key: Some("f84089af-2dc4-4119-b671-e8e297b4dd34".to_string()),
        Speech_URL: Some("wss://services.govivace.com:49153".to_string()),
        Translation_URL: Some("mt.lovoco.co".to_string()),
        Success: true,
        Message: "".to_string()
    };

    let json= r#"{"UserName": "steve@lovoco.co","Password":"123"}"#;

    let url = "https://stenopoly.lovoco.co/api/LoginAPI/WinAppAuthAPI".parse().unwrap();
    rt::run(fetch_url(url, json,b));
}

#[test]
fn test_hyper_client_post_ssl_wrongpasswrd()
{
    let b = Body
    {
        User_Authentication_Key: Some("f84089af-2dc4-4119-b671-e8e297b4dd34".to_string()),
        Speech_URL: Some("wss://services.govivace.com:49153".to_string()),
        Translation_URL: Some("mt.lovoco.co".to_string()),
        Success: true,
        Message: "".to_string()
    };

    let json= r#"{"UserName": "steve@lovoco.co","Password":"XXX"}"#;

    let url = "https://stenopoly.lovoco.co/api/LoginAPI/WinAppAuthAPI".parse().unwrap();
    rt::run(fetch_url(url, json,b));
}


#[test]
fn test_hyper_client_post_ssl_fail()
{
    let b = Body
    {
        User_Authentication_Key: Some("00000000-0000-0000-0000-000000000000".to_string()),
        Speech_URL: None,
        Translation_URL: None,
        Success: false,
        Message: "please enter valid user name".to_string()
    };

    let json= r#"{"UserName": "no_such_user@exist.in.db","Password":"password"}"#;
    let url = "https://stenopoly.lovoco.co/api/LoginAPI/WinAppAuthAPI".parse().unwrap();


    rt::run(fetch_url(url, json,b));
}

fn fetch_url(url: hyper::Uri, json: &'static str, expected_res_body: Body) -> impl Future<Item=(), Error=()>
//fn fetch_url(url: hyper::Uri) -> impl Future<Item=(), Error=()>
{
    //let mut req = Request::new(Body::from(json));
    let mut req = Request::new(hyper::body::Body::from(json));
    //hyper::body

    *req.method_mut() = Method::POST;
    *req.uri_mut() = url.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        //HeaderValue::from_static("application/json")
        hyper::header::HeaderValue::from_static("application/json")
    );

    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    println!("Before client request execution: 1 ");
    //println!("req: {}", req);

    client
        // Fetch the url...
        //.get(url)
        .request(req)
        // And then, if we get a response back...
        .and_then(move | res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());

            // The body is a stream, and for_each returns a new Future
            // when the stream is finished, and calls the closure on
            // each chunk of the body...
            res.into_body().for_each(move |chunk|
            {
                println!("Before client request execution: 2 ");

                let vec = chunk.to_vec();
                let response_json = String::from_utf8(vec).unwrap();
                println!("response_json = {}", response_json);
                let res_body = serialize_response(&response_json).unwrap();

//                println!("res_body.User_Authentication_Key: {}", res_body.user_authentication_key);
//                println!("res_body.Speech_URL: {}", res_body.speech_url);
//                println!("res_body.Translation_URL: {}", res_body.translation_url);

//                println!("res_body.User_Authentication_Key: {}", res_body.User_Authentication_Key);
//                println!("res_body.Speech_URL: {}", res_body.Speech_URL);
//                println!("res_body.Translation_URL: {}", res_body.Translation_URL);

                assert_eq!(res_body.Success, expected_res_body.Success);
                assert_eq!(res_body.User_Authentication_Key, expected_res_body.User_Authentication_Key);
                assert_eq!(res_body.Speech_URL, expected_res_body.Speech_URL);
                assert_eq!(res_body.Translation_URL, expected_res_body.Translation_URL);
                assert_eq!(res_body.Message, expected_res_body.Message);

                //assert_eq!(expected_res_body, res_body);

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
    // Some JSON input data as a &str. Maybe this comes from the user.

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let b: Body = serde_json::from_str(json)?;

    // Do things just like with any other Rust data structure.
//    println!("res_body.User_Authentication_Key: {}", b.User_Authentication_Key);
//    println!("res_body.Speech_URL: {}", b.Speech_URL);
//    println!("res_body.Translation_URL: {}", b.Translation_URL);

    Ok(b)
}




//struct Body
//{
//    userid: u64,
//    user_authentication_key: String,
//    user_access_expiration_date: Option<String>,
//    hours_used: Option<u32>,
//    hours_available: Option<u32>,
//    speech_url: String,
//    translation_url: String,
//    success: bool,
//    message: String
//}

//{
//    "UserID": 53,
//    "User_Authentication_Key": "f84089af-2dc4-4119-b671-e8e297b4dd34",
//    "User_Access_Expiration_Date": null,
//    "Hours_Used": null,
//    "Hours_Available": null,
//    "Speech_URL": "wss://services.govivace.com:49153",
//    "Translation_URL": "mt.lovoco.co",
//    "Success": true,
//    "Message": ""
//}

//#[derive(Serialize, Deserialize)]
//#[derive(PartialEq, Eq)]
//#[serde(rename_all = "lowercase")]
//struct Body
//{
//    //UserID: u64,
//    User_Authentication_Key: String,
//    //User_Access_Expiration_Date: Option<String>,
//    //Hours_Used:  Option<u32>,
//    //Hours_Available:  Option<u32>,
//    Speech_URL: String,
//    Translation_URL: String,
//    Success: bool,
//    Message: String
//}