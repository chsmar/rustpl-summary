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
            fn wrap(&self) {
                /*todo*/
            }
        }
        let gift3 = Gift::Book(String::from("book3"), 230);
        gift3.wrap();
        let gift4 = Gift::Cake(String::from("cake4"));
        gift4.wrap();
        println!("Gift3: {:?}, Gift4: {:?}", gift3, gift4);
    }
}
