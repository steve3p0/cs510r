// Inspired by:
// https://www.w3resource.com/euler-project/euler-problem9.php
pub fn find() -> Option<u32>
{
    for a in 1..1000
    {
        for b in 2..1000
        {
            // I had to use i32 for c because subtracting
            // a and b would overflow (- negative)
            let c:i32 = 1000 - a - b;

            if a * a + b * b == c * c
            {
                // I kind of knew that casting it to u32
                // and therefore losing the sign wouldn't
                // be a problem.  Yeah, it's kind of a hack
                return Some((a * b * c) as u32)
            }
        }
    }

    return None
}

pub fn is_pythagorean(a: u32, b: u32, c: u32) -> bool
{
    a.pow(2) + b.pow(2) == c.pow(2)
}