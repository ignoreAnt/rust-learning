fn main() {
    let mut s = String::from("hello world");

    let my_string = String::from("hello world");

    let word = first_word_v1(&s); // word will get the value 5

    let s = "Hello, world!";

    let word = first_word_v1(&my_string[0..6]);
    let word = first_word_v1(&my_string[..]);

    println!("The first word is: {}", word);
    // s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_v1(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}