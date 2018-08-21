table!
{
    app_users (Id)
    {
        Id -> Integer,
        Username -> Text,
        Password -> Text,
        UserAuthenticationKey -> Text,
        TranslationURL -> Text,
        SpeechURL -> Text,
    }
}

//table!
//{
//    posts (id)
//    {
//        id -> Integer,
//        title -> Text,
//        body -> Text,
//        published -> Bool,
//    }
//}

allow_tables_to_appear_in_same_query!(
    app_users,
    //posts,
);
