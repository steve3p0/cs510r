extern crate regex;
use regex::Regex;

pub fn reply(message: &str) -> &str
{
    let r_special = r"[`~!@#$%&*()_=+{}|;:',.<>/?]";
    let r_special_esc = r"[\^\[\]\-\\]";
    let r_1or_more_exc = r"(!+)";
    let r_0or_more_exc = r"(!*)";

    let msg = message.trim();

    // Just look at Bob weird
    if msg == ""
    { return "Fine. Be that way!"}

    // Yell: CAPS Ask
    {
        let r = r"^([A-Z]+\s*)+(\?+)$";
        if bob_match(&r, msg)
        { return "Calm down, I know what I'm doing!" }
    }
    // Ask: Question Mark
    if msg.ends_with("?")
    { return "Sure." }

    // Yell: CAPS, numbers, punct, and !
    {
        let r_caps_nums_spec = format!(r"(([0-9A-Z]|{}|{})+\s*)+", r_special, r_special_esc);
        let r = format!(r"^{}{}$", &r_caps_nums_spec, r_1or_more_exc);

        if bob_match(&r, msg)
        { return "Whoa, chill out!" }
    }

    // Yell: CAPS and 0 or more !
    {
        let r_caps_spec = format!(r"(([A-Z]|{}|{})+\s*)+", r_special, r_special_esc);
        let r = format!(r"^{}{}$", r_caps_spec, r_0or_more_exc);

        if bob_match(&r, msg)
        { return "Whoa, chill out!" }
    }

    "Whatever."
}

fn bob_match(r: &str, msg: &str) -> bool
{
    //println!("r: {}", r);

    let re = Regex::new(r).unwrap();
    return  re.is_match(msg)
}