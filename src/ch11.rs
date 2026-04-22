// Writing Automated Tests
// 'Correctness' is complex and not easy to prove.
// Testing is a complex skill: how to write good tests: mechanics of testing facilities:
// - annotations and macros
// - default behavior and options provided for running your tests
// - how to organize tests into unit tests and integration tests

// How to Write Tests
// Test body: typically perform these three actions:
// - Set up any needed data or state.
// - Run the code you want to test.
// - Assert that the results are what you expected.

// Structuring Test Functions
// Filename: src/lib.rs
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]   // indicates test function
    fn exploration() {  // function name (test name) is showing in results statistics
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    //#[test]
    //fn another() {  // shows FAILED in results statistics
    //    panic!("Make this test fail");  
    //}
}

// $ cargo test
// ...
// running 1 test
// test ch11::tests::it_works ... ok

// 0 measured; statistic for benchmarks.

// Checking Results with assert!
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        //self.width < other.width && self.height < other.height // wrong logic
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod rtests {
    use super::*;  // the rtests module is an inner module, 
    // we need to bring the code under test in the outer module into the scope of the inner module.
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
        //assert!(!smaller.can_hold(&larger));
    }
}

// Testing Equality with assert_eq! and assert_ne!
#[cfg(test)]
mod eqtests {
    use super::*;
    #[test]
    fn ne_add() {
        assert_ne!(5, add(2, 2));  // 5 != 4 // macros use the operators == and !=
    }
}

// Adding Custom Failure Messages
pub fn wrong_add(left: u64, right: u64) -> u64 {
    left - right
}
#[cfg(test)]
mod cmtests {
    use super::*;
    #[test]
    fn eq_add() {
        assert!(4 == wrong_add(2, 2), "Custom failure message: 4 should be equal to add: 2, 2");  // 4 == 4
    }
}

// Checking for Panics with should_panic
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod sptests {
    use super::*;
    #[test]
    #[should_panic]   // indicates that the test should panic,
    // and the test will pass if the code inside the function panics.
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "bla bla")]
    fn greater_than_100_expected() {
        Guess::new(200);
    }
}

// Using Result<T, E> in Tests
#[cfg(test)]
mod result_tests {
    use super::*;
    #[test]
    fn it_works() -> Result<(), String> {  // Result<>: enables you to use the question mark operator
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}