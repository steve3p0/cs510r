extern crate regex;
use regex::Regex;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool
{
    // match Regex to validate
    println!("1 isbn: {}", isbn);

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
    let re = Regex::new(r"^(\d-\d{3}-\d{5}-(\d|X)|\d{9}(\d|X))$").unwrap();
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

