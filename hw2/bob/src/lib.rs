extern crate regex;
use regex::Regex;

pub fn reply(message: &str) -> &str
{
    let msg = message.trim();

    // Just look at Bob weird
    if msg == ""
    { return "Fine. Be that way!"}

    // Yell Ask
    let mut re = Regex::new(r"^([A-Z]+\s*)+(\?+)$").unwrap();
    if re.is_match(msg)
    { return "Calm down, I know what I'm doing!" }


    // Question Mark
    if msg.ends_with("?")
    { return "Sure." }

    // All Caps
    //^([A-Z]+\s*)+(\!*)$
    re = Regex::new(r"^([A-Z]+\s*)+(!*)$").unwrap();
    if re.is_match(msg)
    { return "Whoa, chill out!" }



    "Whatever."






    //unimplemented!("have Bob reply to the incoming message: {}", message)
}
