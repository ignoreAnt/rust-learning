use rand::Rng;  // Importing the random number generator trait
use std::cmp::Ordering; // Importing the Ordering enum for comparisons
use std::io;
use rand::rngs::ThreadRng;
// Importing input/output functionality

/// A number guessing game.
///
/// The game will generate a random number between 1 and 100,
/// and the user will have to guess the number.
///
/// The user will be prompted to input a number, and the game
/// will continue to prompt the user until the correct number
/// is guessed.
///
/// The game will print out hints to the user, such as "Too big!"
/// or "Too small!" to help the user guess the correct number.
fn main() {
    println!("Guess the number between 1 and 100!");

    // let secret_number = rand::thread_rng().gen_range(1..=100); // Generate a random number between 1 and 100
    // Creating a mutable variable to store the secret number
    
    
    let mut rng = ThreadRng::default(); // this should be done only once
    let secret_number = rng.gen_range(1..=100);
    
    // Loop until the user guesses the correct number
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
