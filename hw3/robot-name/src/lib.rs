extern crate time;
extern crate radix_fmt;

pub struct Robot
{
    name: String
}

impl Robot
{
    pub fn new() -> Robot
    {
        // n[0..2].chars().all(|c| c >= 'A' && c <= 'Z'),
        //n[2..].chars().all(|c| c >= '0' && c <= '9')

        // 26 X 26 = 676
        // 10 x 10 x 10 = 1000
        // 647 x 1000 = 676,000

        // name[0]: A-Z: 26     // month
        // name[1]: A-Z: 26     // day
        // name[2]: 0-9: 10     // year
        // name[3]: 0-9: 10     // hour
        // name[4]: 0-9: 10     // minute

//        let c = 'A';
//        println!("A: {}", c.to_digit());

        // 1533539770.9886765
        //             676000
        //             6 7 6 0 0 0
        //
        //

        // 1533539770.9886765
        // 1533539770.9 8 8 6 7 6 5
        //                        0-9
        //                        A-J

//        let codepoint_array: Vec<u8> = "B".into();
//        //let codepoints: Vec<char> = codepoint_array.into_iter().map(char::from).collect();
//        //println!("{:?}", codepoints);
//
//
//        let point = codepoint_array.get(0);
//        println!("point: {}", point.unwrap());

//        let p1 = prefix + 10; //  + 65;
//        //let c1 = radix_fmt::radix_36(p1).;
//        println!("Base26({}): {}", prefix, c1);
//        println!("Base26({}): {}", prefix, radix_fmt::radix_36(p1)); //radix_fmt(676, 26));
//        //println!("Base26({}): {}", prefix, radix_fmt::radix_26(prefix)); //radix_fmt(676, 26));


        for i in 0..676
        {
            let prefix = base26_alpha(i);
            println!("base26_alpha({}): {}", i, prefix);
        }


        let rnd_name = "AB123".to_string();
        Robot { name: rnd_name }
        //unimplemented!("Not implemented")
    }



    pub fn name<'a>(&'a self) -> &'a str
    {
        &self.name
        //unimplemented!("Not implemented")
    }

    pub fn reset_name(&mut self)
    {
        unimplemented!("Not implemented")
    }
}

// Inspired by:
// https://stackoverflow.com/questions/48983939/convert-a-number-to-base-26
pub fn base26_alpha(mut n: i32) -> String
{
    // 0 = AA
    // 1 = AB
    // 2 = AC
    // 3 = AD
    // 4 = AE

    // 25 = AZ
    // 26 = AA
    // 27 = AB
    // 28 = AC

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
//    else if b == 1
//    {
//        a = a - 1;
//        b = b + 26;
//    }

    (a, b)
}





//Time Stamp 1:  1533539770.9886765
//Time Stamp 2:  1533539770.9886773
//Time Stamp 3:  1533539770.9886777
//Time Stamp 4:  1533539770.9886785
//Time Stamp 5:  1533539770.988679
//Time Stamp 6:  1533539770.9886794
//Time Stamp 7:  1533539770.98868
//Time Stamp 8:  1533539770.9886804
//Time Stamp 9:  1533539770.9886808
//Time Stamp 10: 1533539770.9886813


// Time Stamp: 1533539292.3725164
// Time Stamp: 1533539332.662702
// Time Stamp: 1533539338.824166
// Time Stamp: 1533539376.6954174
// Time Stamp: 1533539434.514298

// 26 X 26 = 676
// 10 x 10 x 10 = 1000
// 647 x 1000 = 676,000

// name[0]: A-Z: 26     // month
// name[1]: A-Z: 26     // day
// name[2]: 0-9: 10     // year
// name[3]: 0-9: 10     // hour
// name[4]: 0-9: 10     // minute

// name[0]: A-Z: 26     // minute
// name[1]: A-Z: 26     // second
// name[2]: 0-9: 10     //
// name[3]: 0-9: 10     //
// name[4]: 0-9: 10     //