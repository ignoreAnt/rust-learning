fn main() {
    // let reference_to_nothing = dangle();

}

fn change(some_string: &String) -> &String {
    &some_string
}

// fn dangle() -> String {
//     let s = String::from("hello");
// 
//     &s
// }