use std::io;   // bring the input/output library into scope.
//             // 'std' By default brings into the scope of every program. 
//             // This set is called the prelude             

pub fn guess() {    // function without parameters and start body
    println!("Guess the number!");   // macro that prints

    println!("Please input your guess.");

    let mut guess = String::new();   // mutate variable that is currently 
    //                               // bound to a new, empty instance of a String.
    //                               // 'new' is an 'associated function' of the String type.

    io::stdin()
        .read_line(&mut guess)  // handle to get input from the user to store in 
        //                      //argument (mutable reference &)
        .expect("Failed to read line"); // Handling Potential Failure, example invalid utf-8 input
    //                                  // $ printf '\xFF\xFE' | ./target/debug/rustpl-summary
    //                                  // 'expect' is a 'method'

    println!("You guessed: {guess}");   // Printing Values with Placeholders {}
}

// add rand crate dependency
// $ cargo add rand
// rand = "0.10.0"   # shorthand for ^0.10.0, which means any version that is at least 0.10.0 but below 0.11.0.
// Cargo fetches the latest versions of everything that dependency needs from the 'registry',
// which is a copy of data from https://crates.io/ 

// Cargo.lock: cargo ensuring reproducible builds with versions specified there
// Updating a Crate: cargo will ignore the Cargo.lock file and figure out all the 
// latest versions that fit your specifications in Cargo.toml.
// $ cargo update

use rand::prelude::*;    // Convenience import of common rand members
use std::cmp::Ordering;  // Bringing Ordering enum into scope

pub fn guess_rand() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);  // Inclusive range

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // Shadowing let us reuse guess variable name
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    //               ^^^^^ original String variable
    // trim(): String method, eliminate any whitespace at the beginning and end.
    //         guess looks like: "5\n", if the user types 5 and presses enter
    // parse(): String method, converts a string to another type
    // Rust will infer that secret_number should be a u32 as well.
    // Comparison will be between two values of the same type!
    // expect(..): Result method, when parse return Err variant, 
    // crash and print the message 

    println!("You guessed: {guess}");
    match guess.cmp(&secret_number) {   // comparing and matching the result
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

// Build documentation provided by all your dependencies locally and open it in your browser.
// $ cargo doc --open 

// Allowing Multiple Guesses with Looping
pub fn guess_loop() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {  // Infinite loop
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // Handling Invalid Input 
            Ok(num) => num,
            Err(_) => continue,  // The underscore, _, is a catch-all value;
            // continue, which tells the program to go to the next iteration of the loop, 
            // the program ignores all errors that parse might encounter!
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;   // exit the loop
            }
        }
    }
}