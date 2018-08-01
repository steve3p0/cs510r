extern crate regex;

use regex::Regex;

pub fn answer(command: &str) -> i64
{


    //let r = r"^([A-Z]+\s*)+(\?+)$";
    //let re = Regex::new(r).unwrap();


    //if  re.is_match(command)

    0

}

pub fn simple_answer(command: &str) -> i64
{
    // Now get the operands
    let pattern = r"\d+";
    let regex = match Regex::new(pattern)
    {
        Ok(r) => r,
        Err(e) => { panic!("Could not compile regex: {}", e); }
    };

    let operands = regex.find_iter(command);

    for o in operands
    {
        println!("operand: {}", o.as_str());
    }

    // Now get the operators
    let pattern = r"plus|minus|multiplied\sby|divided\sby";
    let regex = match Regex::new(pattern)
    {
        Ok(r) => r,
        Err(e) => { panic!("Could not compile regex: {}", e); }
    };

    let operators = regex.find_iter(command);

    for o in operators
    {
        println!("operator: {}", o.as_str());
    }

    // Now get the operators
    let pattern = r"plus|minus|multiplied\sby|divided\sby";

    let ops = get_matches(pattern, command);

    for o in ops
    {
        println!("operator: {}", o);
    }

    0
}

fn get_matches(pattern: &str, s: &str) -> Matches
{
    let regex = match Regex::new(pattern)
    {
        Ok(r) => r,
        Err(e) => { panic!("Could not compile regex: {}", e); }
    };

    let operators = regex.find_iter(s);

    for o in operators
    {
        println!("operator: {}", o.as_str());
    }
}


pub fn simple_answer2(command: &str) -> i64
{

    // get first operand
    let r = r"(?:What\sis\s)([0-9]+)";
    let re = Regex::new(r).unwrap();
    let caps = re.captures(command).unwrap();
    let operand1:i32 = caps.get(1).unwrap().as_str().parse().unwrap();

    println!("operand1: {}", operand1);

//    // Now for the rest
//    let pat = r"/\d+/g";
//    let re = Regex::new(pat).unwrap();
//
//    //let caps = re.captures(command).iter();
//    //let caps1 = re.captures(command);
//    let caps2 = re.captures(command).unwrap();
//
//    for c in caps2.iter()
//    {
//        let cp = c.unwrap().as_str();
//        println!("c: {}", cp);
//    }

    //
//    let pattern = r"plus|minus|multiplied\sby|divided\sby)";
//    let regex = match Regex::new(pattern)
//    {
//        Ok(r) => r,
//        Err(e) => { panic!("Could not compile regex: {}", e); }
//    };
//
//    let result = regex.find_iter(command);
//
//    //for (start, end) in result
//    for m in result
//    {
//        println!("m: {}", m.as_str());
//    }
//


    //(\s(plus|minus|multiplied\sby|divided\sby)\s[0-9]+)+
    // ([0-9])(\s(plus)\s[0-9]+)+

    // (?:\s)(plus)(?:\s)([0-9]+)+
    // (?:(?:\s)(plus)(?:\s)([0-9]+))+
    // ((?:\s)(plus)(?:\s)([0-9]+)+)

    //let r1 = r"(\s(plus|minus|multiplied\sby|divided\sby)\s[0-9]+)+";
    //let r1 = r"(?:(?:\s)(plus)(?:\s)([0-9]+))+";
    //let r1 = r"(?:\s)(plus)(?:\s)([0-9]+)+";
    //let r1 = r"";
    //let r1 = r"((\s)(plus)(\s)([0-9]+)+)";
    //let r1 = r"(plus\s[0-9]+)+";
    //let r1 = r"(plus\s[0-9]+)+";
    //let r1 = r"(plus\s\d+)+";


//    let r1 = r"\d+";
//    let re = Regex::new(r1).unwrap();
//
//    //let caps = re.captures(command).iter();
//    let caps1 = re.captures(command);
//    let caps2 = re.captures(command).unwrap();
//
//    for c in caps2.iter()
//    {
//        let cp = c.unwrap().as_str();
//        println!("c: {}", cp);
//    }

//    let cap1 = caps.get(1).map_or("", |m| m.as_str());
//    let cap2 = caps.get(2).map_or("", |m| m.as_str());
//
//    println!("cap 1: {}", cap1);
//    println!("cap 2: {}", cap2);



//
//    let operand1 = re.captures(command).unwrap().get(1).unwrap().as_str();
//
//    let cap

    //captures(command).unwrap().get(1).unwrap().as_str();
    //captures.get(1).unwrap().as_str()


//    {
//        //Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),
//        //Some(caps) => println!("Found match: {}", &caps[0]),
//        &caps[0],
//        //Some(caps) => &caps[0].borrow(),
//        None => panic!("Can't find operand")
//    };

    //println!("operand1: {}", operand1);


//    // ([0-9])(\s(plus)\s[0-9]+)+
//    let r = r"([0-9])(\s(plus)\s[0-9]+)+";
//
//    let re = Regex::new(r).unwrap();
//
//    let captures = re.captures_iter(command).collect::<Vec<_>>();
//    let tokens: Vec<(&str, &str)> = captures.iter()
//        .flat_map(|t| t.iter_named())
//        .filter(|t| t.1.is_some())
//        .map(|t| (t.0, t.1.unwrap()))
//        .collect();
//
//    for token in tokens
//    {
//        println!("{:?}", token);
//    }

    //let caps = re.captures(com).iter();
//    let caps = re.captures(com).unwrap();
//
//    let cap1 = caps.get(1).map_or("", |m| m.as_str());
//    let cap2 = caps.get(2).map_or("", |m| m.as_str());
//
//    println!("cap 1: {}", cap1);
//    println!("cap 2: {}", cap2);

//    for c in caps
//    {
//        c.
//    }
//
//    if  re.is_match(command)

    0
}

pub fn simple_answer1(command: &str) -> i64
{

    let com = "5 plus 13 plus 6";

    // ([0-9])(\s(plus)\s[0-9]+)+
    let r = r"([0-9])(\s(plus)\s[0-9]+)+";

    let re = Regex::new(r).unwrap();
    //let caps = re.captures(com).iter();
    let caps = re.captures(com).unwrap();

    let cap1 = caps.get(1).map_or("", |m| m.as_str());
    let cap2 = caps.get(2).map_or("", |m| m.as_str());

    println!("cap 1: {}", cap1);
    println!("cap 2: {}", cap2);

//    for c in caps
//    {
//        c.
//    }
//
//    if  re.is_match(command)

    0
}



/// Calculates the Sum
/// Takes the sum of n and m
/// Notes: What asserts are necessary here?
///    - Zero and Negative are valid, correct?
pub fn sum (n: i64, m: i64) -> i64
{
    n + m
}


/// Calculates the Product
/// Takes the product of n and m
/// Notes: What asserts are necessary here?
///    - Zero and Negative are valid, correct?
pub fn product (n: i64, m: i64) -> i64
{
    n * m
}


/// Calculates the Greatest Common Divisor
/// Taken directly from Programming Rust by Jim Blandy, Jason Orendorff
/// Chapter 2, Handling Command-Line Arguments (pg 10) with slight
/// modifications:
///    - (params are i64 instead of u64)  - a common interface for all operations
///    - we change the assert to make no zero or negative values
pub fn gcd(mut n: i64, mut m: i64) -> i64 {
    assert!(n > 0 && m > 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}