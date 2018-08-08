extern crate rand;

use rand::Rng;

pub struct Robot
{
    name: String
}

impl Robot
{
    pub fn new() -> Robot
    {
        let rnd_name = Robot::random();
        Robot { name: rnd_name }
    }

    pub fn name<'a>(&'a self) -> &'a str
    {
        &self.name
    }

    pub fn reset_name(&mut self)
    {
        loop
        {
            let rnd_name = Robot::random();

            if rnd_name != self.name
            {
                self.name = rnd_name;
                break;
            }
        }
    }

    pub fn random() -> String
    {
        let rnd_prefix = rand::thread_rng().gen_range(0, 675);
        let prefix = Robot::base26_alpha(rnd_prefix);
        let rnd_suffix = rand::thread_rng().gen_range(0, 999);
        let mut suffix = rnd_suffix.to_string();

        let rep = 3 - suffix.len();
        let padding = "0".repeat(rep);
        suffix = format!("{}{}", padding, suffix);

        let random = format!("{}{}", prefix, suffix);
        //println!("random: {}", random);

        random
    }

    pub fn base26_alpha(mut n: i32) -> String
    {
        let mut chars: Vec<char> = Vec::new();

        loop
        {
            let (a, b) = Robot::divmod(n);
            n = a;

            let c = (b + 65) as u8 as char;

            chars.push(c);

            if n == 0 { break; }
        }

        if chars.len() < 2
        {
            chars.push('A');
        }

        chars.reverse();
        let hash: String = chars.iter().collect();

        hash
    }

    pub fn divmod(n: i32) -> (i32, i32)
    {
        let mut a = n / 26;
        let mut b= n % 26;

        if b == 26
        {
            a = a - 1;
            b = b + 26;
        }

        (a, b)
    }
}



