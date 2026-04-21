// Generic Types, Traits, and Lifetimes
// Tools for handling abstract stand-ins for concrete types: 'Generics'.
// Explore, How to reduce code duplication in functions
// Combine 'traits' with generics to constrian a generic type to accept only those types that have a particular behavior.
// 'Lifetimes': a variety of generics that give the compiler information about how references relate to each other.

// Removing Duplication by Extracting a Function
// When we have a algorithm to find the largest number in a list,
// we define a function against duplicate the code for every list

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// Steps to extracting a function:
// 1. Identify the duplicated code.
// 2. Extract the duplicated code into a new function, 
//    specifying the inputs and return values in the signature.
// 3. Replace the duplicated code with calls to the new function.

// Generic Data Types
// In Function Definitions:
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// combine largest_i32 and largest_char into a single function 
// that can operate on any list of items that can be compared with each other
// Parameterize the types in a new single function, use 'T' by convention
// Declare the type parameter name before use it, place type name declarations 
// inside angle brackets, <>, between the name of the function and the parameter list
// fn largest<T>(...) ...
// Read as “The function largest is generic over some type T”
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// The code above will not compile because the compiler doesn't know if the type T can be compared
// To enable comparisons, the standard library has the std::cmp::PartialOrd trait.
// The standard library implements PartialOrd on both i32 and char.

// In Struct Definitions:
struct Point<T> {  // declare the name of the type parameter 'T'
    x: T,   // x and y are both same type T
    y: T,
}

// In Enum Definitions
enum Result<T, E> {  // declare the name of the type parameters 'T' and 'E'
    Ok(T),
    Err(E),
}

// In Method Definitions
impl<T> Point<T> {  // declare the name of the type parameter 'T' next to impl
    fn x(&self) -> &T {
        &self.x
    }
}
//In Method Definitions with concrete type
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct PointXY<X1, Y1> {
    x: X1,
    y: Y1,
}
impl<X1, Y1> PointXY<X1, Y1> {
    pub fn mixup<X2, Y2>(self, other: PointXY<X2, Y2>) -> PointXY<X1, Y2> {
        PointXY {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn generic_types() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let _integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // error! no method named `distance_from_origin` found for struct `Point<{integer}>` in the current scope
    // println!("distance from origin for integer point: {}", _integer.distance_from_origin()); 
    println!("distance from origin for float point: {}", float.distance_from_origin());
    let p1 = PointXY { x: 5, y: 10.4 };
    let p2 = PointXY { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("Mixup p1 and p2: p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Performance of Code Using Generics
    // Rust performs 'monomorphization' of the code that uses generics at compile time. The compiler looks at 
    // all the places where generic code is called and generates code for the concrete types the generic code is called with.
    let _integer: Option<i32> = Some(5); // generates an Option_i32
    let _float: Option<f64> = Some(5.0); // generates an Option_f64
}

// Defining Shared Behavior with Traits
// A 'trait' defines the functionality a particular type has and can share with other types.
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.
// Traits are similar to a feature often called interfaces in other languages, although with some differences.
pub fn shared_behavior_with_traits() {
    // Defining a Trait
    // Different types share the same behavior if we can call the same methods on all of those types.
    pub trait Summary {  // 'trait' definition with the name Summary
        fn summarize(&self) -> String; // shared behavior defined by the method summarize
        // other shared behavior can be added here as more methods
    }
    
    // Implementing a Trait on a Type
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    // Implementing the Summary trait for NewsArticle
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{} ({})", self.headline, self.author)
        }
    }
    
    // use some_module::{NewsArticle, Summary};  // to use, include both
    // One restriction to note is that we can implement a trait on a type only if either the 
    // trait or the type, or both, are local to our crate.
    // These restrictions called 'coherence', 'orphan rule': parent type is not present.

    // Using default implementation:
    pub trait Editable {
        fn edit(&self) -> bool { // default implementation
            false
        }
    }
    impl Editable for NewsArticle {} // use default implementation of edit method

    // Using Traits as Parameters
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // Trait Bound Syntax
    pub fn notify2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    // Multiple Trait Bounds with the + Syntax
    pub fn notify3(item: &(impl Summary + Editable)) {
        println!("Breaking news! {}", item.summarize());
    }
    pub fn notify4<T: Summary + Editable>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    // Clearer Trait Bounds with 'where' Clauses
    pub fn notify5<T>(item: &T) 
        where T: Summary + Editable { // 'where' clause after the function signature.
        println!("Breaking news! {}", item.summarize());
    }

    // Returning Types that Implement Traits
    pub fn returns_summarizable() -> impl Summary {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        }
    }
    // fn returns_summarizable(switch: bool) -> impl Summary { // use 'impl' Trait if you’re returning a single type.
    //     if switch {                                         // Returning either a NewsArticle or a Tweet isn’t allowed
    //         NewsArticle {...}
    //     } else {
    //         Tweet {...}
    //     }
    // }

    // Using Trait Bounds to Conditionally Implement Methods
    use std::fmt::Display;
    struct Pair<T> { x: T, y: T }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    let pair = Pair::new(3, 5);
    pair.cmp_display();

    // Implementing a Trait on a Trait: 'blanket implementations'
    // impl<T: Display> ToString for T { 
        //...
    // }
    // we can call the 'to_string' method defined by the 'ToString' trait on 
    // any type that implements the 'Display' trait.
}

// Validating References with Lifetimes
// Every reference in Rust has a lifetime. 
// Most of the time, lifetimes are implicit and inferred.
pub fn lifetimes() {
    // Dangling References:
    //let r;
    //{
    //    let x = 5;
    //    r = &x;
    //}
    //println!("r: {r}"); // An attempt to use a reference whose value has gone out of scope.
    // If Rust allowed this code to work, 'r' would be referencing memory that was deallocated.

    // The Borrow Checker:
    // compares scopes to determine whether all borrows are valid.
    // Annotations of the lifetimes of r and x, named 'a and 'b, respectively:
    //fn f1() {
    //    let r;                // ---------+-- 'a
    //    {                     //          |
    //        let x = 5;        // -+-- 'b  |
    //        r = &x;           //  |       |
    //    }                     // -+       |
    //    println!("r: {r}");   //          |
    //}                         // ---------+
    // The program is rejected because 'b is shorter than 'a: 
    // The subject of the reference doesn’t live as long as the reference.
    fn f1() {
        let x: i32 = 5;            // ----------+-- 'b
                                   //           |
        let r: &i32 = &x;          // --+-- 'a  |
                                   //   |       |
        println!("r: {r}");        // --+       |
    }                              // ----------+
    // Here, x has the lifetime 'b, which in this case is larger than 'a. 
    // This means r can reference x

    // Generic Lifetimes in Functions:
    //fn longest(x: &str, y: &str) -> &str {  // error: missing lifetime specifier
    //    if x.len() > y.len() { x } else { y }
    //}
    // We can’t tell whether the reference being returned refers to 'x' or 'y'.
    // The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of 
    // 'x' and 'y' relate to the lifetime of the return value.

    // Lifetime Annotation Syntax:
    // describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
    //&i32        // a reference
    //&'a i32     // a reference with an explicit lifetime
    //&'a mut i32 // a mutable reference with an explicit lifetime
    // In Function Signatures: 'longest'
    // - declare the generic lifetime parameters inside angle brackets.
    // - The returned reference will be valid as long as both of the parameters are valid.
    // - This is the relationship between lifetimes of the parameters and the return value.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    // The generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y.
    // The returned reference will also be valid for the length (durante) of the smaller of the lifetimes of x and y.
    fn max<'a>(x: &'a i32, y: &'a i32, z: &'a i32) -> &'a i32 {
        if x > y { if x > z { x } else { z } } else { if y > z { y } else { z } }
    }
    {
        let x = 45;
        let result;
        {// inner scope
            let y = 50;
            {// innermost scope
                let z = 40;
                result = max(&x, &y, &z);
                println!("max of x, y, z is {}", result);
            }
            //println!("max of x, y, z is {}", result); // error: `z` does not live long enough
        }
        //println!("max of x, y, z is {}", result); // error: `y` does not live long enough  
    }

    // Relationships
    // When returning a reference from a function, the lifetime parameter for the return type needs to match the 
    // lifetime parameter for one of the parameters. In other case return an owned data type rather than a reference.

    // In Struct Definitions
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    {
        let ie : ImportantExcerpt;
        {
            let novel = String::from("Call me Ishmael. Some years ago...");
            ie = ImportantExcerpt { part: &novel[0..20] };
            println!("important excerpt: {}", ie.part);
        }
        //println!("important excerpt: {}", ie.part); // error: `novel` does borrow later used here
    }

    // Lifetime Elision
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    // The reason this function compiles without lifetime annotations is historical:
    // Rust programmers were entering the same lifetime annotations over and over in particular situations.
    // These patterns are programmed into Rust’s analysis of references, called the 'lifetime elision rules'.
    // Lifetimes on function or method parameters are called 'input lifetimes', and lifetimes on return 
    // values are called 'output lifetimes'.
    // If the compiler gets to the end of the three rules and there are still references for which it can’t figure out 
    // lifetimes, the compiler will stop with an error.
    // Rule 1: the compiler assigns a lifetime parameter to each parameter that’s a reference. 
    // Rule 2: if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
    // "fn foo<'a>(x: &'a i32) -> &'a i32"
    // Rule 3: if there are multiple input lifetime parameters, but one of them is '&self' or '&mut self' because this is a 
    // method, the lifetime of self is assigned to all output lifetime parameters.
    // Appling Rules:
    // fn first_word(s: &str) -> &str   
    // fn first_word<'a>(s: &'a str) -> &str       // Rule 1
    // fn first_word<'a>(s: &'a str) -> &'a str    // Rule 2
    // Now all the references in this function signature have lifetimes, and the compiler can 
    // continue its analysis without needing the programmer to annotate the lifetimes in this function signature.
    // fn longest(x: &str, y: &str) -> &str
    // fn longest<a', b'>(x: &'a str, &'b str) -> &str      // Rule 1
    // Rule 2: doesn't apply
    // Rule 3: doesn't apply
    // Haven’t figured out what the return type’s lifetime is. Error.

    // In Method Definitions
    //impl<'a> ImportantExcerpt<'a> {
    //   fn announce_and_return_part(&self, announcement: &str) -> &str {
    //      println!("Attention please: {announcement}");
    //      self.part
    //   }
    //}
    // because one of the parameters is '&self', the return type gets the lifetime of '&self' (Rule 3)

    // The Static Lifetime ('static)
    // reference can live for the entire duration of the program
    let s01: &'static str = "I have a static lifetime.";

    // Generic Type Parameters, Trait Bounds, and Lifetimes
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display {
        println!("Announcement! {ann}");
        if x.len() > y.len() { x } else { y }
    }
}
