use hello_world_func_macro::hello_world_func;

struct Hello;

hello_world_func!(Hello);

fn main() {
    Hello::hello();
}
