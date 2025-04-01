fn main() {

    // let mut greeting = String::from("Hello");
    let mut greeting = String::from("Hello");
    let b = &greeting;
    greeting.push(' ');  // Adds a space
    greeting.push_str("World!"); // Appends a string slice

    assert_eq!(greeting, "Hello World!");

    println!("{}", greeting.len());
    greeting.truncate(2);
    println!("{:?}", greeting);

    fn greet(name: &str) {  // Accepts both &str and String (due to deref coercion)
        println!("Hello, {}!", name);
    }

    let name = String::from("Alice");
    greet(&name);  // Works with &String
    greet("Bob");  // Works with string literals

    let names = vec!["Alice".to_string(), "Bob".to_string()];

    // This for loop takes ownership (moves) each string out of `names`.
    for name in &names {
        println!("Name: {}", name);
    }

    

    names.iter().for_each(|name| println!("Name: {}", name));


    let mut numbers = vec![1, 2, 3];

    // Mutable borrow: each element can be changed in-place
    for num in &mut numbers {
        *num *= 2;  // Dereference and multiply by 2
    }

    println!("Numbers after modification: {:?}", numbers);

    let arr = [10, 20, 30, 40, 50];
    let mut i = 0;

    // If you mistakenly use `<=`, you'll try arr[5], out of bounds
    while i <= arr.len() {
        // In Rust, this will panic at runtime if i == 5
        println!("{}", arr[i]);
        i += 1;
    }

}