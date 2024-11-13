use rand::Rng;  // Importing the random number generator trait
use std::cmp::Ordering; // Importing the Ordering enum for comparisons
use std::io;  // Importing input/output functionality

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Generate a random number between 1 and 100

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // Creating a mutable empty string to store user input

        io::stdin()  // Read user input
            .read_line(&mut guess)
            .expect("Failed to read line");  // Error handling

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,        // Successfully parsed input to a number
            Err(_) => continue,    // Non-numeric input, prompt user again
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {  // Compare guess with secret number
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  // Exit the loop on correct guess
            }
        }
    }
}
