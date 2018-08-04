extern crate regex;
#[macro_use]
extern crate log;

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
    command: String,
    operations: Vec<Operation>
}

impl WordProblem
{
    pub fn new(command: &str) -> Self
    {
        // Remove the word 'by' - it's worthless in this context
        // So, X divided by Y => X divided Y
        let c = command.replace(" by", "").to_string();
        let o: Vec<Operation> = Vec::new();

        WordProblem { command: c, operations: o }
    }

    pub fn answer(&mut self) -> Result<i32, String>
    {
        // Create patterns used to parse command
        let pattern_prefix = r"What\sis\s(?:(\-?\d+))";
        let pattern_operations = r"(plus|minus|multiplied|divided)\s\-?\d+";
        let pattern = format!(r"{}(\s{})+", pattern_prefix, pattern_operations);

        // Validate the command string
        let re = Regex::new(&pattern).unwrap();
        if !re.is_match(&self.command)
        {
            return Err(format!("The word problem command is invalid: {}", &self.command))
        }

        // Add the first operand to our math operations collection
        let operation = WordProblem::get_operand(self, pattern_prefix);
        self.operations.push(operation );

        // Now get all the other math operations
        WordProblem::get_operations(self, pattern_operations);

        // Do the math!
        Ok(WordProblem::do_math(self))
    }

    fn get_operand(&self, pattern: &str) -> Operation
    {
        let re= match Regex::new(pattern)
        {
            Ok(r) => r,
            Err(e) => { panic!("Could not compile regex: {}", e); }
        };

        let caps = re.captures(&self.command).unwrap();
        let op1 = caps.get(1).unwrap().as_str();

        // Default 1st operand to 'plus'
        let operand = op1.parse::<i32>().unwrap();
        let operator = "plus".to_string();
        let operation = Operation::new(operand, operator);

        debug!("operator1: {}\noperand1: {}\n\n", operation.operator, operation.operand);

        operation
    }

    fn get_operations(&mut self, pattern: &str)
    {
        let re= match Regex::new(pattern)
        {
            Ok(r) => r,
            Err(e) => { panic!("Could not compile regex: {}", e); }
        };

        let matches = re.find_iter(&self.command);

        for m in matches
        {
            let mut s = m.as_str().split_whitespace();
            let operator = s.next().unwrap().to_string();
            let operand = s.next().unwrap().parse::<i32>().unwrap();
            let operation = Operation::new(operand, operator);

            debug!("operation: {}\noperand: {}\noperator: {}\n\n", m.as_str(), operation.operand, operation.operator);

            self.operations.push(operation );
        }
    }

    fn do_math(&self) -> i32
    {
        let mut total = 0;

        for o in &self.operations
        {
            debug!("{} ", total);

            total = match o.operator.as_str()
            {
                "plus" =>  total + o.operand,
                "minus" =>  total - o.operand,
                "multiplied" =>  total * o.operand,
                "divided" =>  total / o.operand,
                _ => panic!("Operator not found: {}", o.operator)
            };

            debug!("{} {} = {}\n", o.operator, o.operand, total);
        }

        debug!("result = {}\n", total);

        total
    }
}
