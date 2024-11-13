use std::env;
use std::str::FromStr;

/// Computes the greatest common divisor (GCD) of one or more
/// non-zero unsigned integers and prints the result to standard output.
///
/// # Usage
/// 
/// `gcd NUMBER ...`
///
/// # Panics
///
/// Panics if any of the inputs are zero.
fn main() {
    // Define a vector to store the command-line arguments
    let mut numbers = Vec::new();
    //Java :  List<String> strings = new ArrayList<>();

    // Iterate over the command-line arguments
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0]; // `d` will hold the result

    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

/// Computes the greatest common divisor (GCD) of two non-zero unsigned integers using Euclid's algorithm.
///
/// # Arguments
///
/// * `n` - An unsigned integer input (must be non-zero).
/// * `m` - Another unsigned integer input (must be non-zero).
///
/// # Returns
///
/// The GCD of `n` and `m`.
///
/// # Panics
///
/// Panics if either `n` or `m` is zero.
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0); // Check inputs are non-zero.

    // Loop until `m` becomes 0, using the swapping method if `m < n`
    while m != 0 {
        // Swap `m` and `n` to ensure `m` is always greater or equal to `n`
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n; // Replace `m` with `m % n` to apply Euclid's algorithm.
    }

    n // GCD is now stored in `n`
}