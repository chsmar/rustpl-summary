// Common Collections
// vector, string, hash map // others: hash set, binary heap, linked list, btree map, btree set, vecdeque
// the data is stored on the heap
// not known at compile time and can grow or shrink as the program runs
// has different capabilities and costs, choose appropriate

use std::vec;

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
    println!("Unicode String: {} byte size: {}, chars: {}", s7, s7.len(), s7.chars().count()); 
    // some UTF-8 word “नमस्ते” (Hindi, Devanagari script) has Byte, Scalar, Grapheme Cluster counts of 18, 6, and 4 respectively.
    // Byte: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // Unicode scalar: ['न', 'म', 'स', '्', 'त', 'े']
    // Grapheme Cluster: ["न", "म", "स्", "ते"]
    // Rust provides different ways of interpreting the raw string data

    // Sclicing Strings:
    let s7s1 = &s7[0..4]; // 4 bytes, 2 characters
    println!("Slice of s7: {}", s7s1);
    // let s7s2 = &s7[0..1]; // 1 bytes, 1/2 character?? // runtime panic!
    // println!("Slice of s7: {}", s7s2); 

    // Iterating over Strings:
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {  // raw bytes
        println!("{b}");
    }
    for c in "नमस्ते".chars() {
        println!("{c}");
    }
    // String complexity: String is a wrapper around a vector of bytes, so it has the same performance characteristics as a vector.
    // programmers have to put more thought into handling UTF-8 data up front.
}

// Hash Maps
// stores a mapping of keys of type 'K' to values of type 'V' using a 'hashing function', which determines how it places these keys and values into memory.
pub fn hash_maps() {
    use std::collections::HashMap;  // It is not included in the prelude.
    let mut scores = HashMap::new(); // create empty hash map
    scores.insert(String::from("Blue"), 10); // insert key-value pairs
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:?}", scores);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);  // copied() to match types resulting
    // let score = scores.get(&team_name).unwrap_or(&0);  // alternative?
    println!("Score for {}: {}", team_name, score);
    let score = scores.get(&team_name); // get value by key, returns Option
    match score {
        Some(score) => println!("Score for {}: {}", team_name, score),
        None => println!("No score for {}", team_name),
    }

    // Iterating over a Hash Map:
    for (key, value) in &scores {
        println!("Iterating...: {}: {}", key, value);
    }

    // Managing Ownership in Hash Maps
    let k = String::from("Favorite color");
    let v = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(k, v);
    // println!("Ownership Key: {}, ownership Value: {}", k, v); // error: k and v values borrowed here after move
    let k = String::from("Favorite color");
    let v = String::from("Blue");
    let mut mapr = HashMap::new();
    mapr.insert(&k, &v); // works, but the hash map only holds references to the data, so 
    // k and v must be valid as long as the hash map is used
    println!("Reference Key: {} reference Value: {}", k, v); 
    
    // Updating a Hash Map: many options:
    // Overwriting a Value
    scores.insert(String::from("Blue"), 10);
    // Adding a Key and Value Only If a Key Isn’t Present
    scores.entry(String::from("Yellow")).or_insert(50);
    // Updating a Value Based on the Old Value
    let y_score = scores.entry(String::from("Yellow")).or_insert(0);
    *y_score += 10; // y_score is a mutable reference to the value in the hash map, so we dereference it to update the value
    println!("Updated Scores: {:?}", scores);

    // Hashing Functions
    // By default, HashMap uses 'SipHash' hashing function.
    // A 'hasher' is a type that implements the 'BuildHasher' trait.

    // exercise 1
    let mut vs = vec![20, 8, 1, 3, 29, 33, 7, 11, 29, 15, 40, 23, 17, 9, 2, 27, 19, 5, 29, 13];
    vs.sort();
    let median = vs[vs.len() / 2];
    let mut mvs = HashMap::new();
    let mut max_v = 0;
    let mut max_k = 0;
    for n in vs {
        let count = mvs.entry(n).or_insert(0);
        *count += 1;
        if *count > max_v {
            max_v = *count;
            max_k = n;
        }
    }
    println!("Median value: {}, mode: {}", median, max_k);
    
    // exercise 2
    let ss = String::from("apple");
    let mut rs = String::new();
    let mut c0: char = ' ';
    let mut i = 0;
    for c in ss.chars() {
        if i == 0 {
            c0 = if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' { 
                rs.push(c);
                'h' 
            } else { c };
        } else {
            rs.push(c);
        } 
        i += 1;
    }
    rs.push('-');
    rs.push(c0);
    rs.push_str("ay");
    println!("Result string: {}", rs);

    // exercise 3
    let mut dmap: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        println!("Enter a Add command:");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        let parts: Vec<&str> = input.splitn(4, ' ').collect();
        if parts.len() != 4 || parts[0] != "Add" || parts[2] != "to" {
            break;
        }
        dmap.entry(parts[3].to_string()).or_insert(Vec::new()).push(parts[1].to_string());
    }
    if dmap.len() > 0 {
        let mut input = String::new();
        println!("Enter a Department for list people in. Empty for all people in the company: ");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input.is_empty() {
            for (dept, people) in &dmap {
                println!("Department: {}:", dept);
                let mut people = people.clone();
                people.sort();
                for p in people {
                    println!("\t{}", p);
                }
            }
        } else {
            match dmap.get(input) {
                Some(people) => {
                    println!("Department: {}:", input);
                    for p in people {
                        println!("\t{}", p);
                    }
                },
                None => println!("No such department: {}", input),
            }
        }
    }
}    