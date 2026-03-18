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
use crate::ch07::office::contability::Account;
//  -  Relative path: starts from the current module: id, 'self' or 'super'
use office::development::Project;
    let account = Account;
    let project = Project;
    println!("Absolute path: crate::ch07::office::contability::Account: {:?}", account);
    println!("Relative path: office::development::Project: {:?}", project);
}

