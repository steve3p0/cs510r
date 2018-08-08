extern crate robot_name as robot;
use std::collections::HashSet;


#[test]
fn test_different_robots_have_different_names() {
    let r1 = robot::Robot::new();
    let r2 = robot::Robot::new();
    assert_ne!(r1.name(), r2.name(), "Robot names should be different");
}


#[test]
fn test_duplicates()
{
    check_duplicates(10);
    check_duplicates(20);
    check_duplicates(30);
    check_duplicates(40);
    check_duplicates(50);
    check_duplicates(100);
    check_duplicates(500);
    check_duplicates(1000);
    check_duplicates(5000);
    check_duplicates(10000);
    check_duplicates(50000);
    check_duplicates(100000);
    check_duplicates(500000);
    check_duplicates(675999);

}

fn check_duplicates(n: i32)
{
    let mut rnd_nums = HashSet::new();

    let tenth = n / 10;
    //println!("1/10: {}", tenth);

    let mut dupes = 0;
    for i in 0..n
    {
        let rnd = robot::Robot::random();
        //let rnd1 = rnd.clone();
        //println!("robot::random(): {}", i, rnd);

        if !rnd_nums.insert(rnd)
        {
            //println!("DUPLICATE: {}", &rnd1);
            dupes += 1;
        }

//        let x = i + 1; //i + 1;
//        if x % tenth == 0
//        {
//            print!("{}%.. ", 10 * (x / tenth));
//            //println!("{}% : {} random numbers", 10 * (x / tenth), x);
//        }
    }

    println!("Duplicate Count: Frequency({}) - {} out of {}",  dupes as f32 / n as f32, dupes, n);
    //assert!(dupes == 0);
}
