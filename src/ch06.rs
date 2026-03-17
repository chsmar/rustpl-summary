// Enums define a type by enumerating its possible variants.
// A enum value is one of a possible set of variants.
pub fn enums() {
    #[derive(Debug)]
    enum Gift {
        Book,
        Cake,
    }
    let gift1 = Gift::Book; // Enum value
    let gift2 = Gift::Cake;
    println!("Gift1: {:?}, Gift2: {:?}", gift1, gift2);
    fn wrap(_gift: &Gift) { 
        /*todo*/
    }
    wrap(&gift1);
    wrap(&gift2);

    struct Participant {
        name: String,
        gift: Gift,
    }
    let par1 = Participant{name:String::from("Participant1"), gift:gift1};
    println!("Participant name: {}, gift: {:?}", par1.name, par1.gift);

    {
        // a wide variety of types embedded in its variants.
        #[derive(Debug)]
        enum Gift {
            Book(String, u16), // title, pages
            Cake(String),      // name
        }
        //
        impl Gift {
            fn wrap(&self) { // define methods
                /*todo*/
            }
        }
        let gift3 = Gift::Book(String::from("book3"), 230);
        gift3.wrap();
        let gift4 = Gift::Cake(String::from("cake4"));
        gift4.wrap();
        println!("Gift3: {:?}, Gift4: {:?}", gift3, gift4);
    }

    // Option Enum
    // Rust does not have nulls, but it does have an enum that can encode the concept of
    // a value being present or absent, it is defined by the standard library
    // enum Option<T> { // <T> Syntax: Generic type parameter
    //     None, Some(T),
    // }
    let present_x: Option<i32> = Option::Some(5);
    let absent_x: Option<i32> = Option::None; 
    // Eliminating the risk of incorrectly assuming a not-null value
    println!("Option: Present: {:?}, Absent: {:?}", present_x, absent_x);
}

// The 'match' Control Flow
pub fn match_control() {
    enum Gift {
        Book,
        Cake,
    }
    fn wrap(gift: &Gift) { 
        match gift {
            Gift::Book => println!("Wrapping a book"),
            Gift::Cake => println!("Wrapping a cake"),
        }
    }
    let gift1 = Gift::Book;
    let gift2 = Gift::Cake;
    wrap(&gift1);
    wrap(&gift2);

    #[derive(Debug)]
    enum Size {Small,Medium,Large}
    { // Patterns That Bind to Values
        
        enum Gift {
            Book,
            Cake(Size),
        }
        fn wrap(gift: &Gift) { 
            match gift {
                Gift::Book => println!("Wrapping a book"),
                Gift::Cake(Size::Small) => println!("Wrapping a specific small cake"),
                Gift::Cake(zixe) => println!("Wrapping a {zixe:?} cake"),
            }     
        }
        let _gift1 = Gift::Book;
        let gift2 = Gift::Cake(Size::Small);
        let _gift3 = Gift::Cake(Size::Medium);
        let gift4 = Gift::Cake(Size::Large);
        wrap(&gift2);
        wrap(&gift4);
    }

    // The Option<T> match Pattern
    fn has_size(x: Option<Size>) {
        match x {
            Some(sixe) => println!("has size {sixe:?}"),
            None => println!("has no size"),
        }
    }
    has_size(Some(Size::Small));
    has_size(None);

    // Matches Are Exhaustive
    fn next_size(x: Option<Size>) -> Option<Size> {
        match x {
            Some(Size::Small) => Some(Size::Medium), // match arm
            Some(Size::Medium) => Some(Size::Large), // match arm
            Some(Size::Large) => None, // match arm 
            // if any match arm pattern missing, the match is not exhaustive: error!
            None => None, // match arm
        }
    }
    println!("Next size: {:?}", next_size(Some(Size::Small)));

    // Catch-All Patterns and the '_' Placeholder
    fn next_size_placeholder(x: Option<Size>) -> Option<Size> {
        match x {
            Some(Size::Small) => Some(Size::Medium), // match arm
            Some(Size::Medium) => Some(Size::Large), // match arm
            _ => None, // match arm  // catch-all pattern
        }
    }
    println!("Next size simple: {:?}", next_size_placeholder(Some(Size::Large)));
}