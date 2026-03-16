// Struct ia a custom data type to group data

pub fn structs() {
    struct Book {  // 'name' describes the data grouped together
        author: String, // field
        title: String,  // field
        year: u16,      // field
    }
    let b1 = Book {  // instance
        author: String::from("Steve Klabnik, Carol Nichols, and Chris Krycho, with contributions from the Rust Community"),
        title: String::from("The Rust Programming Language"),
        year: 2021 };
    println!("Struct access b1.author: \"{}\", b1.title: \"{}\", b1.year: {}", b1.author, b1.title, b1.year);
    // Field init Shorland
    let year: u16 = 2021;
    let author = String::from("Steve Klabnik, Carol Nichols, and Chris Krycho, with contributions from the Rust Community");
    let b2 = Book {
        author,  // Field init Shorland
        title: String::from("The Rust Programming Language - Practice"),
        year };  // Field init Shorland
    // Struct Update Syntax
    let b3 = Book {
        title: String::from("The Rust Programming Language - Solutions"),
        ..b2 };  // Struct Update Syntax <= author: b2.author, year: b2.year
    println!("Book 2 title: {}, Book 3 title: {}", b2.title, b3.title);
    
    //Tuple Structs
    struct TBook(String, String, u16);
    let tb1 = TBook(String::from("Steve Klabnik, Carol Nichols, and Chris Krycho, with contributions from the Rust Community")
    , String::from("The Rust Programming Language")
    , 2021);
    println!("Tuple Struct access: \"{}\", \"{}\", {}", tb1.0, tb1.1, tb1.2);
    // destructure
    let TBook(author, title, year) = tb1;
    println!("Tuple Struct destructure: \"{}\", \"{}\", {}", author, title, year);

    // Unit-Like Structs
    struct UBook;  // similarly to ()
    let _ubook = UBook;  // instance
}

pub fn funcionality() {
    struct Book {  
        author: String, 
        title: String,  
        year: u16,      
    }
    fn pub_eq(book1_year: u16, book2_year: u16) -> bool {
        book1_year == book2_year
    }
    fn pub_eq_tuple(years: (u16, u16)) -> bool {  // pub_eq reimplementation
        years.0 == years.1
    }
    fn pub_eq_struct(book1: &Book, book2: &Book) -> bool { // pub_eq reimplementation
        book1.year == book2.year
    }
    let b1 = Book {  
        author: String::from("author1"), 
        title: String::from("book1"),  
        year: 2021};
    let b2 = Book {  
        author: String::from("author2"), 
        title: String::from("book2"),  
        year: 2022};
    println!("Book1 publication eq Book2: {}", pub_eq(b1.year, b2.year));
    println!("Book1 publication eq Book2 (tuple): {}", pub_eq_tuple((b1.year, b2.year)));
    println!("Book1 publication eq Book2 (struct): {}", pub_eq_struct(&b1, &b2));

    // println!("book1: {b1}");  // Rust: cannot be formatted with the default formatter.
    // Rust: help: the trait `std::fmt::Display` is not implemented for `funcionality::Book`
    // The primitive types implement 'Display' by default because there’s only one way to display them.
    // Tying 'Debug' ouput format {:?}
    // println!("book1: {b1:?}");  // Rust: cannot be formatted using `{:?}` because it doesn't implement `Debug`    
    // Solution: Adding Functionality with Derived Traits
    {
        #[derive(Debug)] // Derived Trait
        struct Book {  
            author: String, 
            title: String,  
            year: u16,      
        }
        let b1 = Book {  
            author: String::from("author1"), 
            title: String::from("book1"),  
            year: 2021};
        println!("book1: {b1:?}"); // println! takes references
        let b2 = Book {  
            author: String::from("author2"), 
            title: String::from("book2"),  
            year: dbg!(2004+5)};  // instantiate and dbg! aren't lazy
        dbg!(&b2);  // dbg! takes ownership, then &b1    
    }
}

pub fn methods() {
    struct Book {  
        author: String, 
        title: String,  
        year: u16,      
    }
    impl Book { // implementation block for Book
        pub fn author(&self) -> &String { // getter method
            // '&self' instead of book1. Short for 'self: &Self'
            // Methods can take ownership of 'self' (is rare)
            // If we wanted to change the instance, use &mut self as the first parameter
            &self.author
        }
    }
    let b1 = Book {
        author: String::from("author1"), 
        title: String::from("book1"),  
        year: 2021};
    let b2 = Book {  
        author: String::from("author2"), 
        title: String::from("book2"),  
        year: 2022};
    let b1_ref = &b1;
    let b1_author = b1_ref.author();
    println!("Book1 author: {}", b1_author);    
    let b2_author = b2.author(); 
    // Rust has a feature called 'automatic referencing and dereferencing'.
    // Calling methods is one of the few places with this behavior.
    println!("Book2 author: {}", b2_author);
    
    impl Book { // Multiple implementation block for Book
        // Everything within this impl block will be associated with the Book type
        fn pub_eq_impl(&self, book2: &Book) -> bool { // pub_eq Method reimplementation
            self.year == book2.year
        }

        fn book3() -> Self { // Associated Functions
            // The 'Self' keywords aliases for the type Book
            Self { author: String::from("author3"), title: String::from("book3"), year: 2023 }
        }
    }
    println!("Book1 publication eq Book2: {}", b1.pub_eq_impl(&b2));
    let b3 = Book::book3();  // The :: syntax is used for both associated functions and 
    // namespaces created by modules.
    println!("Book3 author: {}", b3.author());  
}
