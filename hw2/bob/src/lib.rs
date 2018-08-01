extern crate regex;
use regex::Regex;

pub fn reply(message: &str) -> &str
{
    let msg = message.trim();

    // Just look at Bob weird
    if msg == ""
    { return "Fine. Be that way!"}

    // Yell Ask
    //^([A-Z]+\s*)+(\?+)$
    //^(([0-9A-Z]|[^\P{P}\?])+\s*)+(\?+)$
    let mut re = Regex::new(r"^([A-Z]+\s*)+(\?+)$").unwrap();
    if re.is_match(msg)
    { return "Calm down, I know what I'm doing!" }


    // Question Mark
    if msg.ends_with("?")
    { return "Sure." }

    // Yelling
    // ^([A-Z]+\s*)+(\!*)$
    // (([0-9A-Z]|[^\P{P}\?])+\s*)+(!*)
    //^(([0-9A-Z]|[^\P{P}\?])+\s*)+(!*))$



    // Yelling: numbers, punct, and !
    {
        //let r_special = r"[%\^*@#$()\[\]=+_\-`'&\\/{}|,?!<>~;:.]";
        let r_special = r"[%*@#$()=+_`'&/{}|,?!<>~;:.]";
        let r_special_esc = r"[\^\[\]\-\\]";

        let r_1or_exc = r"(!+)";
        //let r_nums_punc = r"(([0-9A-Z]|[%\^*@#$()\[\]=+_\-`'&\\/{}|,?!<>~;:.])+\s*)+";
        //let r_nums_punc = format!(r"(([0-9A-Z]|{})+\s*)+", r_special);
        let r_nums_punc = format!(r"(([0-9A-Z]|{}|{})+\s*)+", r_special, r_special_esc);

        let r = format!(r"^{}{}$", &r_nums_punc, r_1or_exc);
        //let r = format!(r"^({}{})|({}{})$", r_yell, r_0or_exc, r_yell, r_1or_exc);

        re = Regex::new(&r).unwrap();
        if bob_match(&r, msg)
        { return "Whoa, chill out!" }
    }
//
//    re = Regex::new(&r).unwrap();
//    if re.is_match(msg)
//        { return "Whatever." }
//
//    let r_0or_exc = r"(!*)";
//    let r_1or_exc = r"(!+)";

    //let r_yell = r"(([0-9A-Z]|[^\P{P}\?])+\s*)+";
    // .^$*+?()[{\|

    //%^*@#$(^
    //"
    //let r_spec = r"%\^\*@#\$\(";

    let r_spec = r"%";
    //\^\*@#\$\(]
    //let r_spec = r"!@#\$%\^&\*\(\)_+\-=\[\]{};':\\\|,.<>\/\?";
    let r_yell = r"(([A-Z]|[%\^])+\s*)+";
    let r_0or_exc = r"(!*)";
    //let r_1or_exc = r"(!+)";

    {
        let r = format!(r"^{}{}$", r_yell, r_0or_exc);
        //let r = format!(r"^({}{})|({}{})$", r_yell, r_0or_exc, r_yell, r_1or_exc);

        re = Regex::new(&r).unwrap();
        if bob_match(&r, msg)
        { return "Whoa, chill out!" }
    }


    "Whatever."






    //unimplemented!("have Bob reply to the incoming message: {}", message)
}

fn bob_match(r: &str, msg: &str) -> bool
{
    println!("r: {}", r);

    let re = Regex::new(r).unwrap();
    return  re.is_match(msg)
}