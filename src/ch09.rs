// Error Handling
// Recoverable errors: Result<T, E>
// Unrecoverable errors: panic!, symptom of bug

// Rust app when a panic occurs:
// - 'unwind' de stack (by default)
// - 'abort' the process (small binary), to enable: in Cargo.toml, add:
//      [profile.release]
//      panic = 'abort' 

use std::io::Read;

pub fn unrecoverable_errors() {
    panic!("crash and burn");
    // RUST_BACKTRACE environment variable to get a backtrace of exactly what happened to cause the error. 
    // A backtrace is a list of all the functions that have been called to get to this point.

    let v = vec![1, 2, 3];
    v[99]; // this will cause a panic: index out of bounds
}

pub fn recoverable_errors() {
    use std::fs::File;
    use std::io::ErrorKind;

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file, // open file successfully arm variant //match 1
        Err(error) => match error.kind() { // open file error arm variant
            ErrorKind::NotFound => match File::create("hello.txt") {  // error NotFound kind arm variant //match 2
                Ok(fc) => fc,  // create file successfully arm variant
                Err(e) => panic!("Problem creating the file: {:?}", e), // create file error arm variant //match 3
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    // Alternative:
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error
    let f = File::open("hello.txt").unwrap(); // panic on error, with a default message 
    // all unwrap methods make panic on error
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // panic on error, with a custom message

    // Propagating Errors
    // without 'propagating' the error
    fn read_username_from_file() -> Result<String, std::io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut username = String::new();
        match f.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // The '?' Operator Shortcut
    fn read_username_from_file2() -> Result<String, std::io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut username = String::new();
        f.read_to_string(&mut username)?;
        Ok(username)
    }
    fn read_username_from_file3() -> Result<String, std::io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    fn read_username_from_file4() -> Result<String, std::io::Error> {
        std::fs::read_to_string("hello.txt")
    }
    // Use the '?' operator in functions whose return type is compatible with the '?'
    // is used on: like: read_username_from_file() -> Result<String, std::io::Error>
    // The '?' operator in a function that returns Result, Option, 
    // or another type that implements FromResidual.
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    // Use the '?' operator on 'Option' return types mixed with 'Result' types or vice versa
    // Use methods like the 'ok' method on Result or the 'ok_or' method on Option to do the conversion
    fn last_char_of_first_line2(file: &str) -> Option<char> {
        std::fs::read_to_string(file).ok() // convert Result to Option
            .and_then(|s| s.lines().next().map(|l| l.chars().last()).flatten()) // flatten the nested Option
    }

    // All the main functions we’ve used return '()'.
    // main can also return a Result<(), E>.
    // fn main() -> Result<(), Box<dyn Error>> {
    //     let greeting_file = File::open("hello.txt")?;
    //     Ok(())
    // }
    // The Box<dyn Error> type is a trait object.
    // Box<dyn Error> means “any kind of error”
    // The main function may return any types that implement 
    // the std::process::Termination trait, which contains a 
    // function report that returns an ExitCode.
}
 
// To panic! or Not to panic!
