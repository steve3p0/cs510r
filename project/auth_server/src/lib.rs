#[macro_use]
extern crate diesel;

extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

//use self::diesel::prelude::*;
use models::*;
//use auth_server::*;

use models::NewPost;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &SqliteConnection, title: &str, body: &str) -> usize
{
    use schema::posts;

    let new_post = NewPost
    {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}

pub fn get_user_creds() -> String
{
    //use auth_server::schema::posts::dsl::*;
    use schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    let mut res: String = "".to_string();
    for post in results
        {
            res.push_str(&post.title);
            println!("{}", post.title);
            println!("----------\n");
            println!("{}", post.body);
        }

    println!("posts: res: {}", res);

    res
}
