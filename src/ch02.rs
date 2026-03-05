use std::io;   // bring the input/output library into scope.
//             // 'std' By default brings into the scope of every program. 
//             // This set is called the prelude             

pub fn guess() {    // function without parameters and start body
    println!("Guess the number!");   // macro that prints

    println!("Please input your guess.");

    let mut guess = String::new();   // mutate variable that is currently 
    //                                       // bound to a new, empty instance of a String.

    io::stdin()
        .read_line(&mut guess)  // handle to get input from the user to store in 
        //                           //argument (mutable reference &)
        .expect("Failed to read line"); // Handling Potential Failure, example invalid utf-8 input
    //                                  // $ printf '\xFF\xFE' | ./target/debug/rustpl-summary

    println!("You guessed: {guess}");
}