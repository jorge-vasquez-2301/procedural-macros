// #[public]
// #[derive(Debug, Clone)]
// struct Example {
//     first: String,
//     pub second: u32,
// }

// #[public]
// #[some_attr]
// #[some_other]
// struct Example2(String, pub u32);

// #[public]
// #[derive(Copy)]
// enum Example3 {
//     Case1,
//     Case2(String, u32),
//     Case3 { first: String, second: u32 },
// }

mod example {
    use make_public_macro::public;
    #[public(exclude(fourth, third))]
    struct Example {
        first: String,
        pub second: u32,
        third: bool,
        fourth: String,
    }

    impl Example {
        pub fn new() -> Self {
            Example {
                first: "first".to_string(),
                second: 1,
                third: true,
                fourth: "fourth".to_string(),
            }
        }
    }
}

use crate::example::*;

fn main() {
    let e = Example::new();
    println!("{}", e.first);
    println!("{}", e.second);
    // println!("{}", e.third);
}
