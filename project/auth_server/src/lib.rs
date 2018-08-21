#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate hyper;
extern crate futures;

extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;


pub mod models;
pub mod schema;
//mod schema { infer_schema!("dotenv:DATABASE_URL"); }

use futures::{future, Future, Stream};
use hyper::{Body, Chunk, Client, Method, Request, Response, Server, StatusCode, header};
use hyper::client::HttpConnector;
use hyper::service::service_fn;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use models::*;

use serde_json::Error;
//use models::NewPost;

pub fn establish_connection() -> SqliteConnection
{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn get_credentials(username: &str, password: &str) -> String
{
    //use auth_server::schema::posts::dsl::*;
    use schema::app_users::dsl::*;

    let connection = establish_connection();

    let user: models::AppUser = app_users
        .filter(Username.eq(username))
        .first(&connection)
        .unwrap_or_else(|_| panic!("Unable to find user {}", username));

//    let json = AppUser
//    {
//        UserAuthenticationKey: user.UserAuthenticationKey,
//        TranslationURL: user.TranslationURL,
//        SpeechURL: user.SpeechURL,
//    };

    let json_creds = format!(r#"{{"User_Authentication_Key": "{}", "Speech_URL": "{}", "Translation_URL":{}, "Success":{}, "Message":""}}"#,
                                &user.UserAuthenticationKey,
                                &user.TranslationURL,
                                &user.SpeechURL,
                                true,
    );


    json_creds
}

pub fn serialize_request(json: &str) -> Result<AppUserRequest, Error>
{
    // Some JSON input data as a &str. Maybe this comes from the user.

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.

    println!("Inside serialize!!!");

    println!("BEORE serde::from_str!!!");
    //serde_json::from_slice(v)
    let app_user_req: AppUserRequest = serde_json::from_str(json)?;
    println!("AFTER serde::from_str!!!");

    // Do things just like with any other Rust data structure.
//    println!("res_body.User_Authentication_Key: {}", b.User_Authentication_Key);
//    println!("res_body.Speech_URL: {}", b.Speech_URL);
//    println!("res_body.Translation_URL: {}", b.Translation_URL);

    Ok(app_user_req)
}

pub fn to_upper(req: Request<Body>) -> Body
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

#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub struct AppUserRequest
{
    pub username: Option<String>,
    pub password: Option<String>,
}