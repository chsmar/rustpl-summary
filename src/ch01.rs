// Introduction
// Installation
// $ rustc --version
// $ rustup update    # updating to a newly released version
// $ rustup doc       # local rust documentation

// Hello world
// $ rustc src/ch01.rs
// $ ./ch01    // Hello, world!

fn main() {
    println!("RUSTC Hello, world!");
}

// Hello Cargo -- Rust’s build system and package manager.
// Create and add dependencies
// $ cargo new rustpl-summary
// $ cargo add rand@0.8.5 trpl@0.2.0

// Filename: Cargo.toml
// [package]  # config a package name, version, rust edition
// [dependencies]

// $ cargo build    
// $ cargo run
// Quickly checks your code to make sure it compiles but doesn’t produce an executable
// $ cargo check   

// $ cargo build --release      # to compile it with optimizations.
