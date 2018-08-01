extern crate pythagorean_triplet;

#[test]
fn test_find()
{
    for a in 1..1000
    {
        for b in 2..1000
        {
            let c = 1000 - a - b;

            if a * a + b * b == c * c
            {
                println!("The answer: {}", a * b * c);
                return
            }
        }
    }

    println!("NO!!!")
}

#[test]
fn test_answer()
{
    let trip = pythagorean_triplet::find();
    assert_eq!(trip, Some(31875000));

    //assert_eq!(pythagorean_triplet::find(), Some(31875000));
}

#[test]
fn test_is_pythagorean_true()
{
    assert!(pythagorean_triplet::is_pythagorean(3, 4, 5));
    assert!(pythagorean_triplet::is_pythagorean(5, 12, 13));
    assert!(pythagorean_triplet::is_pythagorean(24, 7, 25));
}

#[test]
fn test_is_pythagorean_false()
{
    assert!(!pythagorean_triplet::is_pythagorean(3, 4, 1));
    assert!(!pythagorean_triplet::is_pythagorean(0, 4, 1));
    assert!(!pythagorean_triplet::is_pythagorean(10, 7, 2));
}
