extern crate auth_server;

use self::auth_server::*;

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