// Task
// You get an array of numbers, return the sum of all of the positives ones.
// 
// Example
// [1, -4, 7, 12] => 1 + 7 + 12 = 20
// 1+7+12=20
// Note
// If there is nothing to sum, the sum is default to 0.

/// Returns the sum of all positive integers in the provided slice.
///
/// # Arguments
///
/// * `arr` - A slice of integers to be processed.
///
/// # Returns
///
/// The sum of all positive integers in the slice. If there are no positive
/// integers, returns 0.
///
/// # Example
///
/// ```
/// let numbers = [1, -4, 7, 12];
/// assert_eq!(positive_sum(&numbers), 20);
/// ```
fn positive_sum(arr: &[i32]) -> i32 { // fn positive_sum: Defines a function named positive_sum.
    // let mut sum = 0;
    // for elem in arr {
    //     if *elem > 0{ 
    //        sum = sum + elem; 
    //     }
    // }
    // 
    // sum
    arr.iter().filter(|&x| *x > 0).sum()
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_positive_sum(){
        let numbers = [1, -4, 7, 12];
        assert_eq!(positive_sum(&numbers), 20);
    }
    
    #[test]
    fn all_negative_sum(){
        let numbers = [-1, -2, -3, -4];
        assert_eq!(positive_sum(&numbers), 0)
    }
    
    #[test]
    fn empty_sum(){
        let numbers: [i32; 0] = [];
        assert_eq!(positive_sum(&numbers), 0)
    }
    
    #[test]
    fn all_positive_sum(){
        let numbers = [1, 2, 3, 4];
        assert_eq!(positive_sum(&numbers), 10)
    }
    
    #[test]
    fn all_zero_sum(){
        let numbers = [0, 0, 0, 0];
        assert_eq!(positive_sum(&numbers), 0)
    }
}

