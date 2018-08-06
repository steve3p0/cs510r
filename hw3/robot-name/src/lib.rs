extern crate time;

use std::collections::HashSet;
use std::hash::Hash;

pub struct Robot
{
    name: String
}

impl Robot
{
    pub fn new() -> Robot
    {
        check_duplicates(200000);

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
        // n[0..2].chars().all(|c| c >= 'A' && c <= 'Z'),
        //n[2..].chars().all(|c| c >= '0' && c <= '9')

        // 26 X 26 = 676
        // 10 x 10 x 10 = 1000
        // 647 x 1000 = 676,000

        let mut nano_u32:u32 = time::get_time().nsec as u32;
        //println!("\nNanoseconds: {}", nano_u32);

        nano_u32 += 1;
        let mut nano_string = right(nano_u32.to_string(), 6);
        //println!("Nanoseconds as string: {}", nano_string);

        let mut prefix_i32 = left(nano_string.clone(), 3).parse::<i32>().unwrap();
        //println!("prefix_i32: {}", prefix_i32);

        if prefix_i32 > 675
            {
                prefix_i32 -= 400;

                nano_string = left(nano_string,3);
                nano_string.push('0');
            }

        let suffix = right(nano_string, 3);
        let prefix_string = base26_alpha(prefix_i32);
        let random = format!("{}{}", prefix_string, suffix);

        random
    }

}




pub fn left(s: String, n: usize) -> String
{
    let len = s.len();
    let mut s_right = s;

    if len < n
    {
        let mut dif = n - len;
        while dif > 0
        {
            s_right.push('0');

            dif += 1;
        }
    }

    s_right.chars().take(n).collect::<String>()
}

pub fn right(s: String, n: usize) -> String
{
    let len = s.len();
    let mut s_right = s;

    if len < n
    {
        let mut dif = n - len;
        while dif > 0
        {
            s_right.push('0');

            dif += 1;
        }
    }

    let start = len - n;
    s_right.chars().skip(start).take(len).collect::<String>()
}

pub fn base26_alpha(mut n: i32) -> String
{
    //let mut n = num; // 123;
    let mut chars: Vec<char> = Vec::new();

    loop
    {
        let (a, b) = divmod(n);
        n = a;

        let c = itoa(b); // b + 65) as u8 as char;

        chars.push(c);

        if n == 0 { break; }
    }

    if chars.len() < 2
    {
        chars.push('A');
    }

    chars.reverse();
    let hash: String = chars.iter().collect();

    //println!("int2ascii({}): {:?}", num, chars);

    hash
}

pub fn itoa(n: i32) -> char
{
    let c = (n + 65) as u8 as char;

    c
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


fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    let duplicate = iter.into_iter().all(move |x| uniq.insert(x));

    println!("DUPLICATION: {}", duplicate );
    duplicate
}


pub fn check_duplicates(n: u32)
{
    let mut hashes: Vec<String> = Vec::new();

    let tenth = n / 10;
    println!("1/10: {}", tenth);
    //let tenth = (0.10 * n) as u32;

    for i in 0..n
    {
        let rnd = Robot::random();
        //println!("base26_alpha({}): {}", i, rnd);
        hashes.push(rnd);

        let x = i + 1; //i + 1;
        if x % tenth == 0
        {
            println!("{}% : {} random numbers", 10 * (x / tenth), x); //, n / i
        }

    }

    if has_unique_elements(hashes) == true
    {
        println!("HAS DUPLICATIONS");
    }
    else
    {
        println!("NO DUPLICATIONS")
    }
}

