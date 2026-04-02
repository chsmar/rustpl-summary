// Common Collections
// vector, string, hash map // others: hash set, binary heap, linked list, btree map, btree set, vecdeque
// the data is stored on the heap
// not known at compile time and can grow or shrink as the program runs
// has different capabilities and costs, choose appropriate

// Vectors: store more than one value in a single data structure that puts all the values next to each other in memory.
// Vectors can only store values of the same type.
// Vectors are implemented using generics: Vec<T>
pub fn vectors() {
    let mut v: Vec<i32> = Vec::new(); // create empty vector of i32
    // specifing the type within angle brackets.
    let mut _v2 = vec![1, 2, 3]; // create vector with initial values
    // vec! macro
    // Updating a vector: push, pop, insert, remove
    v.push(1); // add elements to the vector
    v.push(2);
    v.push(3);
    println!("Vector: {:?}", v); // print the vector

    // Reading elements from a vector: indexing, get method, iterating
    let third = &v[2]; // access element by index, returns reference
    println!("Third element: {}", third);
    match v.get(99) { // access element by index, returns Option
        Some(third) => println!("Third element using get: {}", third),
        None => println!("No third element"),
    }
    // v[99]; // this will panic because it's out of bounds
    let first = &v[0];  // immutable reference 
    // v.push(6);  // Error: cannot borrow `v` as mutable because it is also borrowed as immutable
    // see push parameter types: fn push(&mut self, value: T) -> ()
    println!("The first element is: {first}"); 

    for i in &v { // iterate over references to the elements
        println!("Element: {}", i);
    }
    for i in &mut v { // iterate over mutable references to the elements
        *i += 1; // modify the elements through mutable references. '*' dereference 
    }
    println!("Modified vector: {:?}", v);

    // Storing Multiple Types in a Vector with Enums
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.123),
    ];
    println!("Spreadsheet row: {:?}", row);

    // Dropping a Vector Drops Its Elements
    // when end of the scope: all of its contents are also dropped
}

// Storing UTF-8 Encoded Text with Strings
// implemented as a collection of bytes
// growable, mutable, owned, UTF-8 encoded string type.
pub fn strings() {
    let mut s = String::new(); // create empty string
    let data = "initial contents";
    let s2 = data.to_string(); // create string from string literal
    let s3 = "initial contents".to_string(); // create string from string literal
    let s4 = String::from("initial contents"); // create string from string literal
    println!("String s: {}", s);
    println!("String s2: {}", s2);
    println!("String s3: {}", s3);
    println!("String s4: {}", s4);
    // Updating a String: push, pop, insert, remove
    s.push_str("initial contents"); // add string slice to the string
    s.push('!'); // add a character to the string
    println!("Updated string: {}", s);
    let s5 = s4 + " More text."; // concatenate strings, s4 is moved and can no longer be used
    // the + operator uses the 'add' method, which takes ownership of the left operand (s4) and borrows the right operand (&str)
    // the second parameter can be coherced to &str. see add method signature: fn add(self, s: &str) -> String
    // the implementation is more efficient than copying.
    println!("Concatenated string: {}", s5);
    let s6 = format!("{} - {}", s2, s3); // concatenate strings with format macro, does not take ownership of any string
    println!("Formatted string: {}", s6);
    let s7 = String::from("Здравствуйте");
    println!("Unicode String: {} {}", s7, s7.len());

    // Reading a String: indexing, get method, iterating
    let first_char = s.chars().nth(0); // get first character, returns Option<char>
    println!("First character: {:?}", first_char);
    for c in s.chars() { // iterate over characters
        println!("Character: {}", c); 
    }
}