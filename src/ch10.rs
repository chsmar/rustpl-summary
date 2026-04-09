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

