use panic_to_result_macro::panic_to_result;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// #[panic_to_result]
// fn create_person_with_empty_panic(name: String, age: u32) -> Result<Person, ()> {
//     if age > 30 {
//         panic!();
//     }
//     Person { name, age }
// }

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let actual = create_person("Sam".to_string(), 22).unwrap();

        assert_eq!(actual.name, "Sam".to_string());
        assert_eq!(actual.age, 22);
    }

    // happy path unchanged

    #[test]
    fn should_err_on_invalid_age() {
        let actual = create_person("S".to_string(), 33);

        assert_eq!(
            actual.expect_err("this should be an err"),
            "I hope I die before I get old".to_string()
        );
    }
}
