// Ownership
// Set of rules how manages memory

// The Stack and the Heap
// The stack stores values (known fixed size) in order LIFO, push-pop
// The heap finds an empty spot, marks used and return a pointer: 'allocating'
// The pointer is a known fixed size, store on the stack.
// Stack stores, faster than 'allocating'

// Ownership Rules
//   value - owner.
//   one owner at a time.
//   owner goes out of scope, value dropped.

// Variable Scope
pub fn fn_scope() {  // start scope
    let s = "hello";  // 's' owner - "hello" value
    // s here is valid
    println!("Function scope var s: {s}");
    { // inner scope
        let w = " world!";
        println!("Inner scope var w: {w}");
    }
}  // end scope // s here is not valid

// Memory and Allocation
// String type
// Variables and Data
pub fn string_type() {
    let mut s = String::from("Allocating start! "); // allocating amount of
    // memory on the heap:
    //   - The memory must be requested from allocator (by programmer)
    //   - return memory when done (by Rust rules, function 'drop') 
    // a lot of dinamic operation on s here
    s.push_str(" Allocating end!"); // 
    println!("String type result: {s}");
}

pub fn move_scope() {
    // Interacting with Move
    let a1: char = 'a';  // Bind 'a' to a1
    let a2 = a1;   // Copy 'a' to a2
    println!("Valid char variables a1: '{a1}', and a2: '{a2}'");
    let s1 = String::from("STR"); // bind value (data in heap) to s1 (pointer in stack) 
    let s2 = s1;  // Move: Copy pointer but not data and invalids s1
    // Rust invalids s1 to prevents ' double free'
    //println!("Invalid string s1: {s1}");
    println!("Valid string s2: {s2} ");

    // Scope and Assignment
    let mut str = String::from("val1"); // bind 's' to a String with value 'val1'
    println!("Initial assigment in scope str: {str}");
    str = String::from("val2");  // new assign, 'val1' goes out of scope and run 'drop' 
    println!("'val1' goes out of the scope and run drop(), str: {str}");

    // Clone: copy heap and data
    let s3 = s2.clone();
    println!("Now both are valid String s2: {s2}, and String s3: {s3}");

    // Stack-Only Data: Copy 
    // char type: known size at compile time is stored on the 'stack' nothing on 
    // the 'heap' that is why both, a1 and a2 are valid
    // Copy types - Copy trait
    // Scalar types:
    //   - i8 i16 i32 i64 i128 isize
    //   - u8 u16 u32 u64 u128 usize
    //   - f32 f64
    //   - bool
    //   - char
    // Reference types: 
    let x = 10;
    let r1 = &x;  // reference r1
    let r2 = r1;  // copy reference
    println!("Both reference are valid variable r1: {r1} and r2: {r2}");
    // Compound types: if elements are copy types
    // arrays
    let ns = [1,2,3];
    let ms = ns;    // copy
    println!("Both arrays are valid ns: {:?} and ms: {:?}", ns, ms);
    let tp1 = (1, true, 3.0);
    let tp2 = tp1;   // copy
    println!("Both tuples are valid tp1: {:?} and tp2: {:?}", tp1, tp2);
    // Every definition annotated with #[derive(Copy)] and if its fields are
    // copy types too.
    // Any group of simple scalar values can implement Copy
}

pub fn ownership_function() {
    // Passing values to a function are similar to assign a value
    let vr1 = String::from("VAR1");
    //let vr2 = vr1;           // vr1 is no longer valid
    fn_takes_ownership(vr1);   // vr1 is no longer valid // Move

    let n1 = 1;
    let n2 = n1;  // i32 implements Copy trait  // Copy
    println!("Both are valid n1: {n1} and n2: {n2}");
    fn_makes_copy(n1);    // Copy

    // Returning values can also transfer ownership
    let vr3 = fn_gives_ownership();  // Move return into vr3
    let vr4 = String::from("VAR4");  // vr4 comes into a scope, 
    // bind value (data in heap) to vr4 (pointer in stack) 
    let vr5 = fn_takes_gives_ownership(vr4); // vr4 is moved into fn
    // which also moves return into vr5
    println!("Return values vr3: {vr3}, vr5: {vr5}, invalid vr4");
}  // out of scope

fn fn_takes_ownership(v: String) {
    // do something;
    println!("Function fn_takes_ownership doing something with v: {v}");
}

fn fn_makes_copy(n2: i32) {
    // do something
    println!("Function fn_makes_copy doing something wit n2: {n2}");
}

fn fn_gives_ownership() -> String {
    let v = String::from("GIVE_OWNERSHIP"); // 'v' comes into a scope
    v  // return and moves out
}

fn fn_takes_gives_ownership(v: String) -> String {
    // 'v' comes into a scope
    v  // return and moves out
}

// References and Borrowing
pub fn references_borrowing() {
    // move ownership is tedious if you need the parameter back
    let vr1 = String::from("VAR1");
    let (vr2, val_return2) = fn_tedious_calculate_val(vr1);
    println!("Tedious move ownership, vr2: {vr2} return: {val_return2}");
    // References: using a value without transferring ownership
    let vr3 = String::from("VAR3");
    let val_return3 = fn_calculate_val(&vr3);
    // Reference concept: &vr3 -> vr3 -> String "VAR3"
    println!("Use references, vr3: {vr3} return: {val_return3}");
    // Mutable reference
    let mut vr4 = String::from("VAR4");
    let val_return4 = fn_mod_calculate_val(&mut vr4);
    // Mutable Reference Rule: If you have a mutable reference to a value,
    // you can have no other references to that value.
    // Rule prevents 'data races':
    //   - Two or more pointers access the same data at the same time.
    //   - At least one of the pointers is being used to write to the data.
    //   - There’s no mechanism being used to synchronize access to the data.
    println!("Use mutable references, vr3: {vr4} return: {val_return4}");
    
    // Multiple mutable references, just not simultaneous ones (Mutable or Inmitable)
    let mr1 = &mut vr4;
    // usage and end
    println!("Use mutable reference mr1: {mr1}");
    let r1 = &mut vr4;
    // usage and end
    println!("Use immutable reference r1: {r1}");
    let mr2 = &mut vr4;
    // usage and end
    println!("Use mutable reference mr2: {mr2}");
    { // Inner scope
        let mr3 = &mut vr4;
        // usage and end
        println!("Use in scope mutable reference mr3: {mr3}");
    }
    // println!("Try use mutable reference mr1: {mr1}"); // Error! simultaneous

    // Dangling References
    //fn dangle_ref() -> &String {
    //    let vrd1 = String::from("DANGLE_VAL");
    //    &vrd1  
    //} // out of scope and vrd1 is invalid then &vrd1 refers to invalid: Error!

    // The Rules of References
    //   - At any given time, you can have either one mutable reference or 
    //     any number of immutable references.
    //   - References must always be valid.

}

fn fn_tedious_calculate_val(v: String) -> (String, i32) {
    let n = v.len() as i32 + 10;
    (v, n)
    // (v, v.len() as i32 + 10) // return directly doesn't work because
    //  ^ v moved here and in next position of tuple is invalid 
}

fn fn_calculate_val(v: &String) -> i32 { // v reference (&) String type // Copy, not move
    // v.push_str("_INCREASED");  // error[E0596]: cannot borrow `*some_string` as mutable...
    v.len() as i32 + 10
} // v reference goes out of scope, nothing else

fn fn_mod_calculate_val(v: &mut String) -> i32 { // v reference (&) String type // Copy, not move
    v.push_str("_INCREASED"); 
    v.len() as i32 + 10
}

pub fn slice_type() {
    // A slice is a reference to a contiguous sequence of elements in a collection.
    // Problem: get words from a string
    // first solucion weak work: 
    //   - get end indexes of words (5,12)
    //   - if ws.clear(); // indexes become obsolete
    // Second solucion: slice
    let mut ws = String::from("word1 word10 word100");
    let w1 = &ws[0..5];  // slice 
    let w2 = &ws[6..12]; // slice
    // ws.clear();  // clear makes it happen compile time error
    println!("Slices w1: {w1}, w2: {w2}");
    // w1{ptr,len} -> ws -> String value
    // slice's data structure: Fat pointer: {ptr,len} 16bytes
}
