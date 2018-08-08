extern crate regex;

//use regex::Regex;

pub fn encrypt(input: &str) -> String
{
    let s = fmt_str(input.to_string());
    let (c, r) = get_dimen(&s);

    let square = build_square(c, r, &s);

    println!("{:?}", square);

    for row in square.iter()
    {
        println!("{}", row);
    }

    unimplemented!("Encrypt {:?} using a square code", input)
}

fn build_square(c: usize, r: usize, s: &str) -> Vec<&str>
{
    let mut remainder = s.clone();

    let mut square:Vec<&str> = Vec::new();

    for _i in 0..c
    {
        let mut col_length = c;
        // let mut row = "";

        let mut padding = "".to_string();
        if remainder.len() < r
        {
            col_length = remainder.len();
        }

        let (mut row, mut rem) = remainder.split_at(col_length);

        square.push(row);

        //s = remainder;
        remainder = rem;
    }


    // pad the las
    square
}

pub fn get_dimen(s: &str) -> (usize, usize)
{
    let len = s.len();
    let c = (len as f64).sqrt().ceil() as usize;
    let mut r = c;

    if len % r != 0
    {
        r -= 1;
    }

    println!("len: {}, c: {}, r: {}", len, c, r);

    (c, r)
}

pub fn fmt_str(input: String) -> String
{

    // let mut s = input.to_lowercase().replace(",", "").replace(".", "").replace(" ", "");
    let s = input.to_lowercase().replace(",", "").replace(".", "").replace(" ", "");

    println!("s after formatting: [{}]", s);

    s

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
//    println!("s1: {}", s1); // => "xxxxx xxxxx!"
//
////    let mut re = Regex::new(r"[A-Za-z]");
////    let r = re.unwrap(); //.replace_all("Hello World!", "x");
////    let s1 = &r.to_string();
//
//    println!("{}", s1); // => "xxxxx xxxxx!"

}
