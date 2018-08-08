extern crate regex;

use regex::Regex;

pub fn encrypt(input: &str) -> String
{
    let mut s = fmt_str(input.to_string()); // = &input.replace(" ", "");
    let (c, r) = get_square(input);

    println!("cunt: [{}]", s);



    unimplemented!("Encrypt {:?} using a square code", input)
}

pub fn get_square(s: &str) -> (usize, usize)
{
    let len = s.len();
    let mut c = (len as f64).sqrt().ceil() as usize;
    let mut r = 0;

    if len % c == 0
    {
        r = c;
    }
    else
    {
        r = c - 1;
    }

    println!("len: {}, c: {}, r: {}", len, c, r);

    (c, r)
}

pub fn fmt_str(mut input: String) -> String
{

    let mut v1 = input.to_lowercase().replace(",", "").replace(".", "").replace(" ", "");


    v1

    //.replace(".", "").replace(" ", "").as_str()

//    //[a-z]+
//    // Create patterns used to parse command
//    let p = r"[a-z]";
//
////    // Validate the command string
////    let re = Regex::new(p).unwrap();
////    if !re.is_match(s)
////    {
////        return Err(format!("The word problem command is invalid: {}", &self.command))
////    }
//
////    let re= match Regex::new(p)
////    {
////        Ok(r) => r,
////        Err(e) => { panic!("Could not compile regex: {}", e); }
////    };
////
////    let caps = re.captures(s).unwrap();
////    let s = caps.get(1).unwrap().as_str();
//
//    let re = Regex::new(r"[A-Za-z]").unwrap();
//    let s1 = re.replace(" ", "");
//    println!("cunt fucker: {}", s1); // => "xxxxx xxxxx!"
//
////    let mut re = Regex::new(r"[A-Za-z]");
////    let r = re.unwrap(); //.replace_all("Hello World!", "x");
////    let s1 = &r.to_string();
//
//    println!("{}", s1); // => "xxxxx xxxxx!"

    //"fuck off"
}
