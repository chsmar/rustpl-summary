// Ownership

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
    let s = "hello";  // 
    // s here is valid
}  // end scope // s here is not valid

// Memory and Allocation
// String type
pub fn string_type() {
    let mut s = String::from("Allocating start! "); // allocating amount of
    // memory on the heap:
    //   - The memory must be requested from allocator (by programmer)
    //   - return memory when done (by Rust rules, function 'drop') 
    // a lot of dinamic operation on s here
    s.push_str(" Allocating end!"); // 
    println!("String type result: {s}");
}

// Interacting with Move
pub fn interacting_move() {
    let a1 = 'a';  // Bind 'a' to a1
    let a2 = a1;   // Copy 'a' to a2
    println!("Valid char variables a1: '{a1}', a2: '{a2}'");
    let s1 = String::from("STR"); 
    let s2 = s1;
    //println!("Invalid string s1: {s1}, s2: {s2} ");
}