// Inspired by:
// https://www.wikiwand.com/en/Sieve_of_Eratosthenes#/Algorithm_and_variants
pub fn primes_up_to(upper_bound: u64) -> Vec<u64>
{
    let upper = upper_bound as usize;
    let mut array = vec![1; upper + 1];
    let mut i= 2;

    println!("sqrt({}): {}", upper_bound, (i as f64).sqrt());
    while (i as f64) <= (upper as f64).sqrt()
    {
        if array[i] == 1
        {
            let mut j = i;
            let step = j;
            j += step;
            println!("j: {}", j);
            println!("step: {}", step);

            while j < upper
            {
                array[j] = 0;
                println!("array[{}]: {}", j, array[j]);
                j += step;
            }
        }

        println!();
        i += 1;
    }

    println!("The result is:");
    let mut a: Vec<u64> = Vec::new();

    let mut i = 2;
    while i < upper
    {
        if array[i] == 1
        {
            a.push( i as u64);
        }
        println!("array[{}]: {}", i, array[i]);
        i += 1;
    }

    println!("FINAL: The result is:");

    for x in &a
    {
        println!("{}", x);
    }

    a
}


pub fn step()
{
    let upper_bound = 101;
    let mut array = vec![1; upper_bound + 1];
    let mut i= 2;

    println!("sqrt({}): {}", upper_bound, (i as f64).sqrt());
    //while i < upper_bound
    while (i as f64) <= (upper_bound as f64).sqrt()
    {
        if array[i] == 1
        {
            let mut j = i;
            let step = j;
            j += step;
            println!("j: {}", j);
            println!("step: {}", step);

            while j < upper_bound
            {
                array[j] = 0;
                println!("array[{}]: {}", j, array[j]);
                j += step;
            }
        }

        println!();
        i += 1;
    }

    println!("The result is:");
    let mut a: Vec<i32> = Vec::new();

    let mut i = 2;
    while i <= upper_bound
    {
        if array[i] == 1
        {
            a.push( i as i32);
        }
        println!("array[{}]: {}", i, array[i]);
        i += 1;
    }

    println!("FINAL: The result is:");

    for x in &a
    {
        println!("{}", x);
    }

//    let mut i = 0;
//    while i < a.
//    {
//        println!("a[{}]: {}", i, a[i]);
//        i += 1;
//    }

}



// Let A be an Array of Boolean values indexed by integers 2 to n
// initially all set to true

// for loop not exceeding sqrt of n
//     if A[] is true:
//         for j = i^2, i^2 + 2i, i^2 +3i, ..., not exceeding n:
//             A[j] := false






//let mut a = BitVec::from_elem(10, true);

//    println!("{:?}", std::mem::size_of::<[bool; 1024]>());
//    println!("{:?}", std::mem::size_of::<[u8; 1024]>());


//    let mut a = iter::repeat(Bit { val: true } )
//        .collect::<ArrayVec<_>>()
//        .into_inner()
//        .unwrap_or_else(|_| unreachable!());
//
//
//
//    //let mut a = [Bit { val: true }; 10];
//    //let mut a = [u8: 10];
//
//

