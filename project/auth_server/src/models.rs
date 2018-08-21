use super::schema::app_users;


//use super::schema::posts;
//#[derive(Queryable)]
//pub struct Post
//{
//    pub id: i32,
//    pub title: String,
//    pub body: String,
//    pub published: bool,
//}
//
//#[derive(Insertable)]
//#[table_name = "posts"]
//pub struct NewPost<'a>
//{
//    pub title: &'a str,
//    pub body: &'a str,
//}

//////////////////////

#[derive(Queryable)]
pub struct AppUser
{
    pub Id: i32,
    pub Username: String,
    pub Password: String,
    pub UserAuthenticationKey: String,
    pub TranslationURL: String,
    pub SpeechURL: String,
}

//#[derive(Insertable)]
//#[table_name = "app_users"]
//pub struct NewAppUser<'a>
//{
//    pub Username: &'a str,
//    pub Password: &'a str,
//    pub UserAuthenticationKey: &'a str,
//    pub TranslationURL: &'a str,
//    pub SpeechURL: &'a str,
//}