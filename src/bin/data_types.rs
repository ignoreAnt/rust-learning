fn main() {
    
    // a number defined as a 128-bit integer
    let number:i128 = 128;
    // a number defined as a 8-bit integer
    let num:u8 = 128;

    // a number defined as unsigned 8-bit integer with suffix `u8`
    let x: u8 = 42_u8;
    // a number defined as isize with suffix `isize`
    let y: isize = 1729isize;

    println!("x : {} ; y : {}", x, y);

    // a number defined as hexadecimal with prefix `0x` and suffix `u32`
    let hex = 0xcafeu32; // 51966
    // a number defined as binary with prefix `0b`
    let bin = 0b0010_1010; // 42
    // a number defined as octal with prefix `0o`
    let oct = 0o106; // 70

    println!("hex : {} ; bin : {} ; oct : {}", hex, bin, oct);

    // a big number defined as 32-bit unsigned integer with _ to separate digits
    let big_num = 4_294_967_295_u32;
    // a hex number defined as 32-bit unsigned integer with _ to separate digits
    let hex_with_underscore = 0xffff_ffff_u32;

    println!("big_num : {:?} ; hex_with_underscore : {}", big_num, hex_with_underscore);

    // a byte literal for character 'A'
    let a = b'A'; // Equivalent to 65u8
    // a byte literal for newline
    let newline = b'\n'; // Equivalent to 10u8

    println!("a: {:?} ; newline: {:#?}", a, newline);

    // a float defined as 64-bit negative infinity
    let negative_infinity = f64::NEG_INFINITY;
    // a float defined as 64-bit positive infinity
    let positive_infinity = f64::INFINITY;
    // a float defined as 64-bit NaN
    let nan = f64::NAN;
    
    println!("negative_infinity: {:?} ; positive_infinity: {:?} ; nan: {:?}",
    negative_infinity, positive_infinity, nan);

    // Conversions that are in range for the destination
    assert_eq!(   10_i8  as u16,    10_u16); // in range
    assert_eq!( 2525_u16 as i16,  2525_i16); // in range

    
    assert_eq!(   -1_i16 as i32,    -1_i32); // sign-extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended

    // Conversions that are out of range for the destination
    // produce values that are equivalent to the original modulo 2^N,
    // where N is the width of the destination in bits. This
    // is sometimes called "truncation."
    assert_eq!( 1000_i16 as  u8,   232_u8);
    assert_eq!(65535_u32 as i16,    -1_i16);

    assert_eq!(   -1_i8  as u8,    255_u8);
    assert_eq!(  255_u8  as i8,     -1_i8);
    
    let list: [i32; 5] = [1, 2, 3, 4, 5];
    let largest = largest(&list);

    let tuple = (10, 3.14, "Rust");
    println!("First: {}", tuple.0); // Access by index

    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("{:?}", vec); // [1, 2, 3, 4]


    let result = safe_divide(10, 1).unwrap_or(0.0);
    println!("{}", result); // Prints: 0.0
    
    println!("{}", 2_i128.pow(3));
    
    println!("{}", 0b1011010110001_u16.count_ones());
    
    let wrapping = 200_u8.wrapping_add(100);
    println!("wrapping : {}", wrapping);
    
    // 10 % 3 = 1
    // 11 % 3 = 2
    // 12 % 3 = 0
    
    let saturating = 200_u8.saturating_add(100);
    println!("saturating : {}", saturating);
    
    let overflowing = 200_u8.overflowing_add(100);
    println!("overflowing : {:?}", overflowing);
    
    let floor = 1.567_f32.floor();
    println!("floor : {}", floor);
    
    let ceil = 1.567_f32.ceil();
    println!("ceil : {}", ceil);
    
    let round = 1.467_f32.round();
    println!("round : {}", round);
   
}

fn build_vector() -> Vec<i16> {
    let mut v = Vec::new(); // Type inferred as Vec<i16> from return type
    v.push(10);            // Element inferred as i16
    v.push(20);
    v
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn safe_divide(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        None
    } else {
        Some(a as f64 / b as f64)
    }
}




