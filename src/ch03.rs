// Common Core
// keywords are reserved for use by the language only,
// cannot use these words as names of variables or functions.

// Variables and mutability
pub fn variables() {
    let x = 5;   // immutable variable, advantage: safety and easy concurrency
    // x = 6;          // cannot assign twice to immutable variable
    let mut mx = 5;  // mutable variable
    mx = 6;
    println!("variables x: {x}, mutable mx: {mx}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Constants always immutable and
    // the type of the value must be annotated (u32),
    // set only a constant expression or with a limited set of operations at compile time

    let sx: &str  = "12345";
    let sx: usize = sx.len();   // Shadowing sx, it is not a mutable variable

    println!("constant: {THREE_HOURS_IN_SECONDS}, shadowing sx: {sx}");
}

// Data Types: two subsets: scalar and compound.
// Rust is a statically typed language, know the types of all variables at compile time.
pub fn datatypes() {
    // Scalar
    // Integer: default type i32
    let ix: i8 = -2; // Integer Signed: i8, i16, i32, i64, i128, isize (arch-dependent) 
    // [−(2^(n − 1)) to 2^(n − 1) − 1 inclusive] i8: [-2^7 to 2^7 - 1] [-128 to 127]
    // let ux: u8 = 2;  // Integer Unsigned: u8, u16, u32, u64, u128, usize (arch-dependent)
    // [0 to 2^n − 1] u8: [0 to 2^8 - 1] [0 to 255]
    let ux = 2u8;  // suffix type u8
    // let vx: u8 = 256;  // 256 Integer overflow, compile time error: literal out of range for `u8`
    // let vx: u8 = ux + 254; // 256 Integer overflow, debug runtime error: panic!
    // println!("Integer overflow vx: {vx}"); // --release mode print: 0 (two’s complement wrapping)
    let mx = 1_000; // Decimal literal same value as 1000 
    println!("integer datatypes: ix: {ix}, ux: {ux}, mx: {mx}");
    // other literals
    let hx = 0xff;  // Hex literal
    let ox = 0o77;  // Octal literal
    let bx = 0b1111_0000;  // Binary literal
    let tx = b'A';  // Byte literal (u8 only)
    println!("literal integer datatypes hx: {hx}, ox: {ox}, bx: {bx}, tx: {tx}");

    // Floating: default type f64
    let rx: f32 = 2.0;

    // Numeric Operations
    let sumx = ux + 2;
    let subx = 33.3 - rx;
    let prdx = 4 * 1_000;
    let divx = -5/3;    // truncate -1.6666666666666667 to -1
    let remx = 43 % 5;
    println!("Operations: addition: {sumx}, subtraction: {subx}, multiplication: {prdx}");
    println!("Operations: division: {divx}, remainder: {remx}");

    // Boolean
    let b: bool = true;  // false

    // Character
    let c: char = 'c'; // 4 bytes 
    // Unicode scalar values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
    println!("Boolean b: {b}, Character c: {c}");

    // Compound
    // Tuple
    let tp: (i32, f64, char) = (500, 6.4, 'a');  // fixed length: cannot grow or shrink in size.
    let (d1,d2,d3) = tp;  // 'destructuring' pattern turn into  three variables
    let tp0 = tp.0;  // access a first tuple element directly by the index
    let utp = ();  // 'unit': a tuple without any values
    println!("Tuples: destructuring tp: ({d1}, {d2}, {d3}), index 0: {tp0}, unit: {:?}", utp);

    // Array
    let ris: [i8; 3] = [1,2,3];  // elements must have the same type. fixed length.
    let r0s = [0; 3];  // initialize [0,0,0]
    let last_ris = ris[2];   // element access result: 3
    let i3: usize = ux as usize + 1;
    // let vi3 = ris[3];  // compile time error: index out of bounds
    // let vi3 = ris[i3];  // runtime panic!: index out of bounds
    println!("Arrays: ris: {:?}, ris[2]: {last_ris}, wrong index: {i3}, initialize: {:?}", ris, r0s);
}

pub fn functions() {
    fn print_fn(b: bool) { // parameters: special variables, part of function's signature
        //                 // always with type                   
        println!("start funtion: {b}");  
    }
    // 'Statements' instructions that perform some action and do not return a value.
    // 'Expressions' evaluate to a resultant value.
    let a6 = 6;  // '6' is a 'Expression' but the line is a 'Statement'
    // let b6 = (let a6 = 6); // that's way you can't assign to 'b6'
    let pr = print_fn(true);  // call function is a 'Expression' even that not defines return
    // let s = fn some_fn() {};  // Function definition is a 'Statement' not a 'Expression'
    //                           // that's way you can't assign to 's'
    let br = {};  // {} block is a 'Expression'
    println!("function return: {:?}, block return: {:?}", pr, br);

    // Function Return
    fn think() -> char {  
        '5'  // expression eq to statement: return '5';
    }
}

//  Coment
/*
    Comments
*/

pub fn controlflow() {
    let condition_exp = 5 > 3;
    if condition_exp {
        println!("condition was true");
    } else {   // optional else
        println!("condition was false");
    }
    let c = 'C';
    if c == 'A' {
        println!("A found!");
    } else if c == 'B' {
        println!("B found!");
    } // multiple condition
}