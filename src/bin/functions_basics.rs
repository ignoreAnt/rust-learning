/// Adds two integers and returns the result.
///
/// # Arguments
///
/// * `a` - The first integer.
/// * `b` - The second integer.
///
/// # Returns
///
/// The sum of `a` and `b`.
fn add(a: i32, b: i32) -> i32 {
    a + b
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
    if n == 0 && m == 0 {
        return 0;
    }
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

fn main() {

    // Testing `add` function with `a = 1` and `b = 2`
    let a = 1;
    let b: i32 = 2;
    // let _c = add(a, b);
    assert_eq!(add(a, b), 3);
    println!("adding a = {} & b = {} equals {}",a, b, add(a, b));

    // Testing `gcd` function with `m = 8` and `n = 12`
    let m = 8;
    let n = 12;
    let result = gcd(m, n);
    println!("gcd of {} and {} is equal to {}",m,n, result);
}

/// Tests the `add` function.
///
/// Verifies that the function correctly adds two integers.
#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

/// Tests the `gcd` function.
///
/// Verifies that the function correctly computes the greatest common divisor
/// of two non-zero unsigned integers.
#[test]
fn test_gcd() {
    assert_eq!(gcd(8, 12), 4); 
    assert_eq!(gcd(12, 8), 4);
    assert_eq!(gcd(12, 12), 12);
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}


#[cfg(test)] // Ensures test code is compiled only in test mode
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);
        assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    }
}
