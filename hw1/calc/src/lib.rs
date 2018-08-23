
/// Calculate actually applies the function opertion (sum, lcm, gcd...)
/// and applies it to the numbers provided.
pub fn calculate(operation: &str, numbers:Vec<i64>) -> i64 {

    let mut d = numbers[0];

    // It would be nice if we could just evaluate the operation string
    // parameter and convert it to a function call, like eval(operation, args)
    match operation {
        "sum" => {
            for m in &numbers[1..] {
                d = sum(d, *m);
            }
        },
        "product" => {
            for m in &numbers[1..] {
                d = product(d, *m);
            }
        },
        "gcd" => {
            for m in &numbers[1..] {
                d = gcd(d, *m);
            }
        }
        "lcm" => {
            for m in &numbers[1..] {
                d = lcm(d, *m);
            }
        }
        // Need to issue an error msg
        // Should I throw an Exception here?
        _ => {},
    }

    d
}

/// Calculates the Greatest Common Divisor
/// Taken directly from Programming Rust by Jim Blandy, Jason Orendorff
/// Chapter 2, Handling Command-Line Arguments (pg 10) with slight
/// modifications:
///    - (params are i64 instead of u64)  - a common interface for all operations
///    - we change the assert to make no zero or negative values
pub fn gcd(mut n: i64, mut m: i64) -> i64 {
    assert!(n > 0 && m > 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

/// Calculates Least Common Multiple
/// Simply multiply n and m and divide that by the gcd
/// Reference:
///     https://www.calculatorsoup.com/calculators/math/lcm.php
pub fn lcm(n: i64, m: i64) -> i64 {
    assert!(n > 0 && m > 0);
    (n * m) / gcd(n, m)
}

/// Calculates the Sum
/// Takes the sum of n and m
/// Notes: What asserts are necessary here?
///    - Zero and Negative are valid, correct?
pub fn sum (n: i64, m: i64) -> i64 {
    n + m
}


/// Calculates the Product
/// Takes the product of n and m
/// Notes: What asserts are necessary here?
///    - Zero and Negative are valid, correct?
pub fn product (n: i64, m: i64) -> i64 {
    n * m
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the GCD Method
    #[test]
    fn test_gcd() {
        // Taken directly from Programming Rust by Jim Blandy, Jason Orendorff
        // Chapter 2, Writing and Running Unit Tests (pg 11)
        assert_eq!(gcd(2 * 5 * 11 * 17,3 * 7 * 13 * 19), 1);
        assert_eq!(gcd(2 * 3 * 5 * 11 * 17,3 * 7 * 11 * 13 * 19), 3 * 11);

        // Test using a vector of numbers
        let numbers: Vec<i64> = vec![799459, 28823, 27347];
        let mut d:i64 = numbers[0];
        for m in &numbers[1..] {
            d = gcd(d, *m);
        }
        assert_eq!(d, 41);
    }

    #[test]
    #[should_panic]
    fn test_gcd_zero1() {
        gcd(5,0);
    }

    #[test]
    #[should_panic]
    fn test_gcd_zero2() {
        gcd(0,5);
    }

    #[test]
    #[should_panic]
    fn test_gcd_zero3() {
        gcd(0,0);
    }

    #[test]
    #[should_panic]
    fn test_gcd_zero4() {
        let numbers: Vec<i64> = vec![5, 15, 25, 35, 0];

        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = gcd(d, *m);
        }
    }

    #[test]
    #[should_panic]
    fn test_gcd_zero5() {
        let numbers: Vec<i64> = vec![5, 15, 0, 35, 25];

        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = gcd(d, *m);
        }
    }

    #[test]
    #[should_panic]
    fn test_gcd_neg1() {
        gcd(-5,5);
    }

    #[test]
    #[should_panic]
    fn test_gcd_neg2() {
        gcd(5,-3);
    }

    #[test]
    #[should_panic]
    fn test_gcd_neg3() {
        gcd(-11,-5550);
    }

    // Test edge cases
    #[test]
    #[should_panic]
    fn test_gcd_edge() {

        // Attempt overflow
        gcd(std::i64::MAX + 1, std::i64::MAX + 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(6, 12), 12);
        assert_eq!(lcm(3, 4), 12);
        assert_eq!(lcm(6, 10), 30);
        assert_eq!(lcm(15, 25), 75);

        let numbers: Vec<i64> = vec![5, 15, 25, 35];

        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = lcm(d, *m);
        }

        assert_eq!(d, 525);
    }

    #[test]
    #[should_panic]
    fn test_lcm_zero1() {
        let numbers: Vec<i64> = vec![5, 15, 0, 35, 25];

        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = lcm(d, *m);
        }
    }

    #[test]
    #[should_panic]
    fn test_lcm_zero2() {
        let numbers: Vec<i64> = vec![0, 15, 0, 35, 25];

        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = lcm(d, *m);
        }
    }

    // Test edge cases
    #[test]
    #[should_panic]
    fn test_lcm_edge2() {

        // Attempt overflow
        // THIS WILL OVERFLOW - In this case
        // LCM of n and m is n x m
        lcm(std::i64::MAX, std::i64::MAX + 1);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(1, 1), 2);
        assert_eq!(sum(2, 2), 4);
        assert_eq!(sum(3, 3), 6);
        assert_eq!(sum(1, 0), 1);
        assert_eq!(sum(0, 13), 13);
        assert_eq!(sum(-5, 10), 5);

        let numbers: Vec<i64> = vec![5, 15, 25, 35];
        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = sum(d, *m);
        }
        assert_eq!(d, 80);

        let numbers: Vec<i64> = vec![5, 15, 25, 35];
        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = sum(d, *m);
        }
        assert_eq!(d, 80);
    }

    // Test edge cases
    #[test]
    #[should_panic]
    fn test_sum_edge1() {

        // Attempt overflow
        sum(std::i64::MAX + 1, std::i64::MAX + 1);
    }

    // Test edge case 2: Sum of two 2 max i64
    #[test]
    #[should_panic]
    fn test_sum_edge2() {

        // Attempt overflow
        sum(std::i64::MAX - 100, std::i64::MAX - 100);
    }

    // Test edge case 2: Sum of two 2 max i64
    #[test]
    #[should_panic]
    fn test_sum_edge3() {

        // Attempt overflow
        sum(-std::i64::MAX, -std::i64::MAX);
    }

    #[test]
    fn test_product() {
        assert_eq!(product(1, 1), 1);
        assert_eq!(product(2, 2), 4);
        assert_eq!(product(3, 3), 9);
        assert_eq!(product(1, 0), 0);
        assert_eq!(product(-5, 10), -50);

        let numbers: Vec<i64> = vec![2, 3, 4, 5];

        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = product(d, *m);
        }

        assert_eq!(d, 120);
    }

    // Test edge cases
    #[test]
    #[should_panic]
    fn test_product_edge1() {

        // Attempt overflow
        // THIS WILL OVERFLOW
        product(std::i64::MAX, std::i64::MAX);
    }

    // Test edge cases
    #[test]
    #[should_panic]
    fn test_product_edge2() {

        // Attempt overflow
        // THIS WILL OVERFLOW
        product(-std::i64::MAX, std::i64::MAX);
    }


}