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
    
}