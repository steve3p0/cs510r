use super::schema::app_users;

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

// For later when this API inserts users
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