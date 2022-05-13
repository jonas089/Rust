pub mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
        fn seat_at_table(){}
    }

    mod serving{
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}

mod back_of_house{
    pub enum Appetizer{
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative
    front_of_house::hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
