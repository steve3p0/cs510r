// Inspired by:
// https://www.wikiwand.com/en/Sieve_of_Eratosthenes#/Algorithm_and_variants
pub fn primes_up_to(upper_bound: u64) -> Vec<u64>
{
    let upper = upper_bound as usize;
    let mut array = vec![1; upper + 1];
    let mut i= 2;

    while (i as f64) <= (upper as f64).sqrt()
    {
        if array[i] == 1
        {
            let mut j = i;
            let step = j;
            j += step;

            while j <= upper
            {
                array[j] = 0;
                j += step;
            }
        }

        i += 1;
    }

    let mut a: Vec<u64> = Vec::new();

    let mut i = 2;
    while i <= upper
    {
        if array[i] == 1
        {
            a.push( i as u64);
        }

        i += 1;
    }

    a
}
