extern crate calc;

use std::io::Write;
use std::str::FromStr;

// Main function of Calculate program
fn main() {

    // Processing of command line args is inspired by
    // Programming Rust by Jim Blandy, Jason Orendorff
    // Chapter 2, Handling Command-Line Arguments (pg 12-13)

    let args: Vec<_> = std::env::args().collect();
    let argc: u32 = args.len() as u32;

    if argc < 2 {
        writeln!(std::io::stderr(), "Usage: OPERATION NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let ref operation= *args[1];
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(2) {
        numbers.push(i64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: OPERATION NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let nums = format!("{:?}", numbers);
    let result = calc::calculate(operation, numbers);

    println!("The {} of {:?} is {}", operation, nums, result);
}