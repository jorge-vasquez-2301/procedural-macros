use make_public_macro::public;

#[public]
struct Example {
    first: String,
    pub second: u32,
}

#[public]
struct Example2(String, pub u32);

fn main() {}
