use make_public_macro::public;

#[public]
#[derive(Debug, Clone)]
struct Example {
    first: String,
    pub second: u32,
}

#[public]
#[some_attr]
#[some_other]
struct Example2(String, pub u32);

#[public]
#[derive(Copy)]
enum Example3 {
    Case1,
    Case2(String, u32),
    Case3 { first: String, second: u32 },
}

fn main() {}
