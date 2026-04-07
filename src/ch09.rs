// Error Handling
// Recoverable errors: Result<T, E>
// Unrecoverable errors: panic!, symptom of bug

// Rust app when a panic occurs:
// - 'unwind' de stack (by default)
// - 'abort' the process (small binary), to enable: in Cargo.toml, add:
//      [profile.release]
//      panic = 'abort' 

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
}