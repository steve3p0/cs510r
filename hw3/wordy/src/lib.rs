extern crate regex;

use regex::Regex;

pub struct Operation
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

pub struct WordProblem
{
    command: String

}

impl WordProblem
{
    pub fn new(command: &str) -> Self
    {
        // Remove the word 'by' - it's worthless in this context
        // 10 divided by 5 => 10 divided 5
        let c = command.replace(" by", "").to_string();

        WordProblem { command: c }
    }

    pub fn answer(&self) -> Result<i32, String>
    {
        // Vector for getting the Operations: operator operand
        let mut operations: Vec<Operation> = Vec::new();

        // Get first operand
        let pattern = r"What\sis\s(?:(\-?\d+))";

        let operation = WordProblem::get_operand(pattern, &self.command);
        //operations.push(operation );

        // Now get the operands
        let pattern = r"(plus|minus|multiplied|divided)\s\-?\d+";
        operations = WordProblem::get_operations(operations, pattern, &self.command);

        Ok(WordProblem::do_math(operation.operand, operations))
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

//        println!("operator1: {}", operation.operator);
//        println!("operand1: {}", operation.operand);
//        println!();

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

    fn do_math(x: i32, operations: Vec<Operation>) -> i32
    {
        let mut total = x;

        for o in &operations
        {
            print!("{} ", total);

            total = match o.operator.as_str()
            {
                "plus" =>  total + o.operand,
                "minus" =>  total - o.operand,
                "multiplied" =>  total * o.operand,
                "divided" =>  total / o.operand,
                _ => panic!("Operator not found: {}", o.operator)
            };

            print!("{} ", o.operator);
            print!("{} ", o.operand);
            println!("= {}", total);
        }

        println!("Grand Total = {}", total);
        total
    }
}





