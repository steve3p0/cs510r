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

use futures::{future, Future, Stream};
use hyper::{Body, Chunk, Client, Method, Request, Response, Server, StatusCode, header};
use hyper::client::HttpConnector;
use hyper::service::service_fn;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use models::*;

use serde_json::Error;

pub fn establish_connection() -> SqliteConnection
{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/// Get Credentials
/// Authenticate the user and lookup it's URLs and auth keys
pub fn get_credentials(username: &str, password: &str) -> String
{
    use schema::app_users::dsl::*;

    let connection = establish_connection();

    // I should probably create an AppUser object and set these
    // fields, then deserialize to json.
    // Didn't have time.
    let invalid_user_msg = r#"
    {
        "UserID": 0,
        "User_Authentication_Key": "00000000-0000-0000-0000-000000000000",
        "User_Access_Expiration_Date": null,
        "Hours_Used": null,
        "Hours_Available": null,
        "Speech_URL": null,
        "Translation_URL": null,
        "Success": false,
        "Message": "please enter valid user name"
    }
    "#.to_string();

    let invalid_pass_msg = r#"
    {
        "UserID": 0,
        "User_Authentication_Key": "00000000-0000-0000-0000-000000000000",
        "User_Access_Expiration_Date": null,
        "Hours_Used": null,
        "Hours_Available": null,
        "Speech_URL": null,
        "Translation_URL": null,
        "Success": false,
        "Message": "please enter valid user name"
    }
    "#.to_string();


    ///"Message": "please enter valid password"
    let user: models::AppUser = app_users
        .filter(Username.eq(username))
        .filter(Password.eq(password))
        .first(&connection)
        //please enter valid user name
        .unwrap_or_else(| _| panic!("Unable to find user {}", username));
        //.unwrap_or_else(| _| invalid_user_msg );

//    if user.Password != password
//    {
//        invalid_pass_msg
//    }

    let json_creds = format!(
        r#"{{"User_Authentication_Key": "{}", "Speech_URL": "{}", "Translation_URL":{}, "Success":{}, "Message":""}}"#,
                &user.UserAuthenticationKey,
                &user.TranslationURL,
                &user.SpeechURL,
                true,
    );

    json_creds
}

pub fn serialize_request(json: &str) -> Result<AppUserRequest, Error>
{
    let app_user_req: AppUserRequest = serde_json::from_str(json)?;

    Ok(app_user_req)
}


#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub struct AppUserRequest
{
    pub username: Option<String>,
    pub password: Option<String>,
}