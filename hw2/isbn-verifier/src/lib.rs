extern crate regex;
use regex::Regex;

/// Determines whether the supplied string is a valid ISBN number

//let text = "3-598-21508-9";
//let text = "3-598-21507-X";
//let text = "3598215088";
//let text = "3-598-21508-8";
pub fn is_valid_isbn(isbn: &str) -> bool
{
    // match Regex to validate
    if match_isbn(isbn)
    {
        //let mut last: u8 = 0;
        let (s , last) = split_x(isbn);

        does_sum_mod11_eq0(s, last)
    }
    else
    {
        false
    }
}

fn does_sum_mod11_eq0(digits: &str, last: u16) -> bool
{
    //digits = digits.chars().rev().enumerate();
    let mut total = last;

    for (i, c) in digits.replace("-", "").chars().rev().enumerate()
    {
        let x = c.to_digit(10).unwrap() as u16;
        let mult = (i + 2) as u16;
        let term = x * mult;

        total += term;
    }

    total % 11 == 0
}

fn match_isbn(s: &str) -> bool
{
    let re = Regex::new(r"(\d-\d{3}-\d{5}-(\d|X))|(\d{9})(\d|X)").unwrap();
    re.is_match(s)
}

fn split_x(mut s: &str) -> (&str, u16)
{
    let last = s.chars().last().unwrap();
    s = &s[..(s.len() - 1)];

    if last == 'X'
    {
        (s, 10)
    }
    else
    {
        (s, last.to_digit(10).unwrap() as u16)
    }
}


////match re.captures(s)
//match re.is_match(s)
//{
////Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),
////Some(caps) => println!("Found match: {}", &caps[0]),
//Some(caps) => true,
//None => panic!("match_isbn_regex: Invalid ISBN format: {}", s)
//}
//unimplemented!("Is {:?} a valid ISBN number?", isbn);

//fn validate_isbn(s: &str) -> bool
//{
//    // get the last char and remove it
//
//    let last = s.chars().nth(s.len() - 1);
//    //let mut digits = s.replace("-", "").chars().rev().enumerate();
//    let mut digits = s.replace("-", "").chars().rev().enumerate();
//
//    //let mut digits = isbn.chars().enumerate();
//    //let mut digits = isbn.chars().take(s.len() - 1);
//    let mut total = 0;
//
//
//    //let x10 = digits.nth(0).unwrap();
//    //let x10 = str.chars().nth(1)
//    if last == Some('X')
//    {
//        total = 10;
//    }
//    else
//    {
//        //let x: u32 = digits.nth(0).unwrap().to_digit(10).unwrap();
//        //last.to_
//        last.to_digist
//        total = x as u32;
//    }
//
//    digits.skip(1);
//
//    for (i, c) in digits.enumerate()
//    {
//        let x: u32 = c.to_digit(10).unwrap();
//        total += x * ((i + 1) as u32);
//    }
//
//    total % 11 == 0
//}


//pub fn convert_to_char()


//    let i:u8 = 10;
//
//    for c in isbn.chars().collect()
//    {
//        match c
//        {
//            Some(c) => println!("Character at index 4: {}", c),
//            None => println!("No character at index 4.")
//        }
//    }
//
//    println!("the fucking char is: {:?}", c);

//    let isbn_s = isbn.to_string();
//
//    let c1 = isbn_s.chars().nth(1) as u8;
//
//
//    let d = isbn_s[1].u8;
