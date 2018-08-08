
pub fn encrypt(input: &str) -> String
{
    if input == ""
    {
        return "".to_string()
    }

    let s = fmt_str(input.to_string());
    let (c, r) = get_dimen(&s);
    let square = build_square(c, r, s);
    let cipher= transponse(c, square);

    cipher
}

fn transponse(c:usize, v: Vec<String>) -> String
{
    let mut s = "".to_string();

    for i in 0..c
    {
        for row in v.iter()
        {
            let ch = row.chars().nth(i).unwrap();
            s.push(ch);
        }

        if i < c - 1
        {
            s.push(' ');
        }
    }

    s
}

fn build_square(c: usize, r: usize, s: String) -> Vec<String>
{
    let mut remainder = s;

    let mut square:Vec<String> = Vec::new();

    for i in 0..r
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

    (c, r)
}

pub fn fmt_str(input: String) -> String
{
    let s = input.to_lowercase().replace(",", "").replace(".", "").replace("-", "")
        .replace(" ", "").replace('\n', "");
    s
}
