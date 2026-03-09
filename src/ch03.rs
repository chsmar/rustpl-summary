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

    // multiple condition
    let x = 'X';
    if x > 'A' {
        println!("Only executes condition A!");
    } else if x > 'B' {
        println!("This never even X > B!");
    } 

    // if in a let Statement
    let v = if x == 'X' {[x]} else {[' ']}; // array same size always
    // else here is required

    // loop
    use rand::prelude::*;
    let mut rng = rand::rng();
    let mut counter = 0;
    let res = loop {
        counter += 1;
        let r = rng.random_range(1..=50);
        if r == 3 {
            break counter+1;
        }
    };
    println!("Let statement v: {:?}, loop res: {res}", v);

    // Loop Labels
    let mut rv = 20;
    'outloop: loop {
        loop {
            if rv < 1 {
                break 'outloop;  // will exit the outer loop
            }
            if rv < 5 {
                break;  // will exit the inner loop
            }
            rv -= 1;
            if rv > 9 {
                continue;
            }
            rv -= 1;
        }
        rv -= 1;
    }
    println!("Loop label result: {rv}");
    
    // Loops with while
    let mut a: char = rng.random();
    print!("while start: {a}");
    while a != 'a' && a != 'A' && !a.is_numeric() && !a.is_whitespace() {
        a = rng.random();
        print!(" {a}");
    }
    println!();

    // Looping for
    for e in "abcd".chars().rev() {
        println!("For loop Char: '{e}'");
    }
}

pub fn practice() {
    fn fahrenheit_celsius(f: f32) -> f32 {
        (f - 32.0)*5./9.
    }
    println!("100 ºf to ºc: {:?}", fahrenheit_celsius(100.0));

    fn fib(n: u128) -> u128 {
        fn fib11(a: u128, b: u128, n: u128) -> u128 {
            if n == 0 {
                0
            } else if n < 3 {
                1
            } else if n == 3 {
                a + b
            } else {
                fib11(b, a+b, n-1)
            }
        }  
        fib11(1,1,n)
    }
    println!("Fibonacci 180th: {}", fib(180));

    println!("\nThe Twelve Days of Christmas\n");
    let gs = ["a partridge in a pear tree","Two turtle doves","Three French hens","four calling birds","Five gold rings","Six geese a-laying","Seven swans a-swimming","Eight maids a-milking","Nine ladies dancing","Ten lords a-leaping","Eleven pipers piping","Twelve drummers drumming"];
    let mut gn = 1;
    for d in ["First","Second","Third","Fourth","Fifth","Sixth","Seventh","Eighth","Ninth","Tenth","Eleventh","Twelfth"] {
        println!("On the {d} day of Christmas my true love sent to me");
        for i in 0..gn {
            let j = i+1;
            if j == gn && gn > 1 {
                print!("And ");
            }
            println!("{}", gs[gn-j]);
        }
        println!();
        gn += 1;
    }
}