extern crate regex;

use regex::Regex;

struct Operation
{
    operand: i32,
    operator: String,
}

impl Operation
{
    fn new(operand: i32, operator: String) -> Self
    {
        Operation { operand: operand, operator }
    }
}

//pub plus()

pub fn answer(command: &str) -> i64
{
    // Remove the word by - it's worthless in this context
    let s = command.replace(" by", "");

    // Vector for getting the Operations: operator operand
    let mut operations: Vec<Operation> = Vec::new();

    // Get first operand

    // What\sis\s(?:([0-9]+))
    // What\sis\s(?:\d+)
    //let pattern = r"What\sis\s(?:\-?\d+)";
    //let pattern = r"What\sis\s(?:([0-9]+))";
    //let pattern = r"What\sis\s(?:(\d+))";
    let pattern = r"What\sis\s(?:(\-?\d+))";

    let operation = get_operand(pattern, &s);
    operations.push(operation );

    // Now get the operands
    //(plus|minus|multiplied\sby|divided\sby)
    let pattern = r"(plus|minus|multiplied|divided)\s\-?\d+";
    operations = get_operations(operations, pattern, &s);

    0
}

fn get_operand(pattern: &str, s: &str) -> Operation
{
    let re= match Regex::new(pattern)
    {
        Ok(r) => r,
        Err(e) => { panic!("Could not compile regex: {}", e); }
    };

    let caps = re.captures(s).unwrap();
    let op1 = caps.get(1).unwrap().as_str();

    // Default 1st operand to 'plus'
    let operand = op1.parse::<i32>().unwrap();
    let operator = "plus".to_string();
    let operation = Operation::new(operand, operator);

    //println!("operation1: {}", m.as_str());
    println!("operator1: {}", operation.operator);
    println!("operand1: {}", operation.operand);
    println!();

    operation
}

fn get_operations(mut operations: Vec<Operation>, pattern: &str, s: &str) -> Vec<Operation>
{
    let re= match Regex::new(pattern)
    {
        Ok(r) => r,
        Err(e) => { panic!("Could not compile regex: {}", e); }
    };

    let matches = re.find_iter(s);

    for m in matches
    {
        let mut s = m.as_str().split_whitespace();
        let operator = s.next().unwrap().to_string();
        let operand = s.next().unwrap().parse::<i32>().unwrap();
        let operation = Operation::new(operand, operator);

        println!("operation: {}", m.as_str());
        println!("operand: {}", operation.operand);
        println!("operator: {}", operation.operator);
        println!();

        operations.push(operation );
    }

    operations
}


