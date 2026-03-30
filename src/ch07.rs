// Packages, Crates, and Modules
// Organizing code by grouping related functionality and separating code with distinct features
// A 'package' can contain multiple 'binary crates' and optionally one 'library crate'.
// Encapsulating implementation: let reuse code at a higher level, defining which parts are 'public' 
// for other code to use and which parts are 'private' implementation.
// Scope: set of names in a nested context

// Binary crate (root: src/main.rs, src/bin/main2.rs,..) <- module*
// Library crate or "crate" (root: src/lib.rs) <- module*

// A 'package' (Cargo.toml) <- (Binary crate)* (Library crate)?
//  - name: package1

// A quick reference on how modules, paths, the 'use' keyword, and the 'pub' keyword work in the compiler
//  - root crate (main.rs, lib.rs, main2.rs...): 
//      - name: same name as the package: package1
//      - 'mod md1;'  // Module: compiler look for in: inline, src/md1.rs, src/md1/mod.rs
//          - 'mod sub_md1;'  // Submodule: compiler look for in: inline, src/md1/sub_md1.rs, src/md1/sub_md1/mod.rs
// Paths to code in module:
//  - 'crate::md1::sub_mod1::Struct1' // if all path items are public
// Modules are private by default
//  - 'pub mod md1;'  // public module
// The 'use' keyword: creates shortcuts to items to reduce repetition of long paths.
//    'use crate::md1::sub_mod1::Struct1;'   // use type in scope 

// package1
// ├── Cargo.lock
// ├── Cargo.toml
// └── src
//     ├── main.rs
//     ├── md1.rs
//     └── md1
//         └── sub_md1.rs

mod sub_ch07 {  // inline submodule
    pub mod sub_sub_ch07 {  // inline submodule
        #[derive(Debug)]
        pub struct Struct1;
    }
}

use crate::ch07::sub_ch07::sub_sub_ch07::Struct1;

pub fn use_module() {
    let struct1 = Struct1;
    println!("Use types in Module: {:?}", struct1);
}

// Grouping Related Code in Modules
mod office { // office es parent of contability, development, sales
    pub mod contability {   // contability nests or child of office
            #[derive(Debug)]
            pub struct Account;
    }
    pub mod development {   // development nests or child of office
            #[derive(Debug)]
            pub struct Project;
    }
    mod sales {         // sales nests or child of office

    }
}
// contability, development, sales are siblings defined within office
// The entire module tree is rooted under the implicit module named 'crate'.

// Path: referring to an item in the module tree
pub fn paths_referring() {
    //  - Absolute path: begins with the crate name, from current crate 'crate'
    let account = crate::ch07::office::contability::Account;
    //  -  Relative path: starts from the current module: id, 'self' or 'super'
    let project = office::development::Project;
    println!("Absolute path: crate::ch07::office::contability::Account: {:?}", account);
    println!("Relative path: office::development::Project: {:?}", project);

    // 'super' use, see in remote_work1 function.
    home::remote_work1();
    // Making Structs and Enums Public, see in describe_house1 function.
    describe_house1();
}
// By default, child modules hide their implementation details, 
// but child modules can see the context in which they’re defined.
// To expose inner parts of child modules’ code to outer ancestor modules by 
// using the 'pub' keyword to make an item public.
// let project = office::development::Project; // works because 'development' and 'Project' are public
// 'office' not need to be public because we are in the same module (ch07).
// Same rule for absolute path.

// Best Practices for Packages with a Binary and a Library
//  - A package with a binary and a library crate is a common pattern in Rust.
//  - The library crate contains the core functionality of the package, 
//    while the binary crate provides a command-line interface or executable that uses the library.
//  - The binary crate becomes a user, a first client of the public API of the library crate, 
//    design a good API!

// Starting Relative Paths with 'super'
// 'super' is like '..' in a file system.
mod home {
    pub fn remote_work1() {
        let project = super::office::development::Project; // 'super' to access 'office' from 'home'
        println!("Use 'super' to access parent module: office::development::Project: {:?}", project);
    }

    // Making Structs and Enums Public
    pub struct House {
        pub name: String,  // some fields of a struct can be public, 
        indoor_color: String, 
    }
    impl House {
        pub fn new(name: String) -> Self {
            Self { name, indoor_color: String::from("white")}
        }
    }
    #[derive(Debug)]
    pub enum FamilyType {
        Nuclear,
        Extended,
        SingleParent,
    }
}

// Making Structs and Enums Public
fn describe_house1() {
    let house1 = home::House::new(String::from("My House"));
    println!("House name: {}", house1.name); // works because 'name' is public
    // println!("House indoor color: {}", house1.indoor_color); // error because 'indoor_color' is private
    println!("Family types: {:?}, {:?}, {:?}", home::FamilyType::Nuclear
    , home::FamilyType::Extended, home::FamilyType::SingleParent); // works because 'FamilyType' and its variants are public
}

pub fn use_keyword() {
    // use home::FamilyType; // bring FamilyType into scope
    use crate::ch07::home::FamilyType; // bring FamilyType into scope
    println!("Family types: {:?}, {:?}, {:?}", FamilyType::Nuclear
    , FamilyType::Extended, FamilyType::SingleParent); 
}


