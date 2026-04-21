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
    }
}

