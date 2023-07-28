#[macro_use]
extern crate hello_world_macro;

#[derive(Hello)]
struct Example;

impl Example {
    fn another_function(&self) {
        println!("Something else");
    }
}

#[derive(HelloAlt)]
enum Pet {
    Cat,
}

fn main() {
    let e = Example;
    e.hello_world();
    e.another_function();

    let p = Pet::Cat;
    p.hello_world();
}
