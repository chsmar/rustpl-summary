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

use rand::prelude::*;

pub fn guessRand() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}