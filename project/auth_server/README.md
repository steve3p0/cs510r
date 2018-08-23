# Auth Server - Web API 
This is a simple web api to authenticate a username and password stored in a sqlite database on a web server.  If a user is authenticated via this web api, additional credentials are passed back to allow a user to access resources. 

##### Acknowledgements: 
This project was heavily influenced by the following projects:
- <a href="https://github.com/actix/examples/tree/master/basics">Actix / Examples / Basics</a><br/>
  An example web server API serving up HTTP POST requests.
   
- <a href="https://github.com/hyperium/hyper/tree/master/examples">hyperium / hyper</a><br/>
  I used this as a reference to test HTTP POST requests

- <a href="https://github.com/hyperium/hyper/tree/master/examples">Diesel - Getting Started</a><br/>
  Examples of using Diesel as an ORM database tool for performing CRUD in rust


##### Setup: Install Rust/Dependencies
```bash
sudo apt-get update
curl https://sh.rustup.rs -sSf | sh
sudo apt-get cargo
sudo apt-get install sqlite3 libsqlite3-dev
cargo install diesel_cli --no-default-features --features sqlite
git clone https://github.com/steve3p0/cs510r.git
```

##### To Run the Web Server:
```bash
cd cs510r/project/auth_server
cargo run
```

You should see the following output after running "cargo run":
```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/auth_server`
Listening on http://127.0.0.1:1337
```

##### To Run the Tests (in another terminal):
```bash
cd cs510r/project/auth_server
cargo test
```
###### NOTE ABOUT TESTING: YOU MUST CHECK THE OUTPUT

Integration tests are located in test/client_hyper_post.rs.
These tests unfortunately will always pass!  You need to check the output. 
Please read the comments for each test to understand what the output should be. 

#### Design Considerations
The biggest challenge was just actually writing tests for HTTP POST in rust.  You have to make a 
futures call in order to execute an HTTP post/get command.  So you setup your HTTP POST, then you
pass it off to a futures call.  There is no way to signal back from the future what the result of that
HTTP Post is.  So the asserts are embedded in the futures call, and because of this, 
the test won't panic when an assert fails.  Any panic! triggered by a failed assertion inside these 
futures calls, won't cause the test to fail.
I tried to have the future call return data
so I could assert the values after (and outside of) the futures call.  I don't see any mechanism for this at all.

Additionally, I tried to pass in params by reference to the futures call so that I could set those params after the HTTP POST.  
My thinking was that since those params are passed by reference, I could evaluate (assert) the results after the call. 
These future calls are performed in the same thread (it has been observed that is performed in a synchronous manner)
so returning a value or passing a result by reference SHOULD be trivial.  Unfortunately, with rust, it's not trivial. 

I kept getting errors stating that my param passed by reference "error[E0597]: 'param' does not live long enough". 
From what I can tell, this is by design.  I found no reference to help get around this. 
There are some things I can still try.  There is something called a Telegram client that if implemented in rust may resolve this, 
but that will require further research.

If you want to test this outside of rust, make sure that the web server is running (cargo run) and then execute the following curl command at a terminal command line:

```bash
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"username":"joeblow","password":"password"}' \
  http://127.0.0.1:1337/api/LoginAPI/WinAppAuthAPI 
```

This should produce the following output:
```bash
{"User_Authentication_Key": "authkey123", "Speech_URL": "wss://asr.acme.com:12345", "Translation_URL":"mt1.lovoco.co", "Success":true, "Message":""}
```

