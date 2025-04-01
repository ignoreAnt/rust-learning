fn main() {
    let x: f32 = 1.0 / 3.0;

    let y: f64 = 1.0 / 3.0;

    println!("f32: {x}, f64: {y}");


    println!("Infinity: {}", f32::INFINITY);
    println!("Infinity: {}", f32::INFINITY + 1.0);

    println!("NaN + 1 = {:?}", f64::NAN + 1.0);

    let num: f32 = -3.75;

    println!("Floor: {}, Ceil: {}, Round: {}", num.floor(), num.ceil(), num.round());


    println!("Is Palindrome: {}", is_palindrome("mom"));
    println!("malayalam:  {}" , is_palindrome("malayalam"));

    let t = ("Brazil", 1985,);
    println!("Country: {}, Year: {}", t.0, t.1); // Accessing tuple elements

    let not_a_tuple = ("single",);
    println!("{}", not_a_tuple.0);

}


fn is_palindrome(word: &str) -> bool {

    let reversed: String = word.chars().rev().collect();

    word == reversed

}
