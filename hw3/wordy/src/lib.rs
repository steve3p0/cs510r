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
        // So, X divided by Y => X divided Y
        let c = command.replace(" by", "").to_string();

        WordProblem { command: c }
    }

    pub fn answer(&self) -> Result<i32, String>
    {
        // Get first operand
        let pattern_prefix = r"What\sis\s(?:(\-?\d+))";
        // Now get the operands
        let pattern_operations = r"(plus|minus|multiplied|divided)\s\-?\d+";

        let pattern = format!(r"{}(\s{})+", pattern_prefix, pattern_operations);

        let re = Regex::new(&pattern).unwrap();
        //let re = Regex::new(&pattern_prefix).unwrap();
        if !re.is_match(&self.command)
        {
            return Err(format!("The word problem command is invalid: {}", &self.command))
        }

        let operation = WordProblem::get_operand(pattern_prefix, &self.command);

        // Vector for getting the Operations: operator operand
        let mut operations: Vec<Operation> = Vec::new();
        operations.push(operation );
        operations = WordProblem::get_operations(operations, pattern_operations, &self.command);

        Ok(WordProblem::do_math(operations))
    }

    fn get_operand(pattern: &str, s: &str) -> Operation
    {
        //let shit = "";

        let re= match Regex::new(pattern)
        //let re= match Regex::new(shit)
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

    fn do_math(operations: Vec<Operation>) -> i32
    {
        let mut total = 0;

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
