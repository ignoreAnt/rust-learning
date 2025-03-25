
fn main() {
    let s = String::from("Hello, world!");

    let result = move_string(s);
    println!("{}", result);
    // println!("{}", s); // This will cause a compile-time error if uncommented
}

fn move_string(s: String) -> String {
    s
}