extern crate regex;

//use regex::Regex;

pub fn encrypt(input: &str) -> String
{
    if input == ""
    {
        return "".to_string()
    }

    let s = fmt_str(input.to_string());
    let (c, r) = get_dimen(&s);

    let square = build_square(c, r, s);

    println!("Read into a square: {:?}", square);

    for row in square.iter()
    {
        println!("{}", row);
    }

    let cipher = square.join(""); //.into_iter().collect();

    println!("before cipher: [{}]", cipher);
    println!("before length: {}", cipher.len());

    let cipher= transponse(c, square);

    println!("after cipher: [{}]", cipher);
    println!("after length: {}", cipher.len());
    cipher

    //unimplemented!("Encrypt {:?} using a square code", input)
}

fn transponse(c:usize, v: Vec<String>) -> String
{
    let mut s = "".to_string(); //String::from_str("");
    //let col = c - 1;

    for i in 0..c
    {
        for row in v.iter()
        {
            let ch = row.chars().nth(i).unwrap();
            s.push(ch);

            print!("{}", row.chars().nth(i).unwrap());
        }

        if i < c - 1
        {
            s.push(' ');
        }

        println!();
    }

    s
}

fn build_square(c: usize, r: usize, s: String) -> Vec<String>
{
    let mut remainder = s;

    let mut square:Vec<String> = Vec::new();

    for i in 0..r //r-1 //c
    {
        let mut col_length = c;
        let mut spaces = "".to_string();

        if remainder.len() != 0
        {
            if remainder.len() < r && i == r - 1
            {
                spaces = " ".repeat(col_length - remainder.len());
                col_length = remainder.len();
            }

            let mut row = remainder[0..col_length].to_string();
            let rem = remainder[col_length..remainder.len()].to_string();

            remainder = rem;
            row = [row, spaces].join("");

            square.push(row);
        }
    }

    square
}

pub fn get_dimen(s: &str) -> (usize, usize)
{
    let len = s.len();
    let c = (len as f64).sqrt().ceil() as usize;
    let mut r = c;

    if len % r != 0
    {
        r -= 1;
    }

    println!("len: {}, c: {}, r: {}", len, c, r);

    (c, r)
}

pub fn fmt_str(input: String) -> String
{
    let s = input.to_lowercase().replace(",", "").replace(".", "").replace("-", "")
        .replace(" ", "").replace('\n', "");

    println!("s after formatting: [{}]", s);

    s
}
