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

allow_tables_to_appear_in_same_query!(
    app_users,
);
