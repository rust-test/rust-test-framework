#[cfg(test)]
mod tests {
    use rust_test_framework::test_case_source;
    use serde::Deserialize;
    use std::any::{type_name, type_name_of_val};

    #[allow(dead_code)]
    #[derive(Deserialize)]
    pub struct User {
        pub name: String,
        pub age: u32,
    }

    #[test_case_source(JsonFile("tests/test_ddt_data.json"))]
    fn test_users(user: User) {
        println!("User age: {}", user.age);
        assert!(user.age > 0);
    }

    #[test_case_source(JsonFile("tests/test_built_in_types_u32.json"))]
    fn test_built_in_types_u32(number: u32) {
        assert_eq!(type_name_of_val(&number), type_name::<u32>());
    }

    #[test_case_source(JsonFile("tests/test_built_in_types_string.json"))]
    fn test_built_in_types_string(string: String) {
        assert!(!string.is_empty());
    }

    #[test_case_source(JsonFile::<u32>("tests/test_generic.json"))]
    fn test_generic<T>(val: T)
    where
        T: std::fmt::Debug,
    {
        let debug_string = format!("{:?}", val);
        println!("{}", debug_string);
        println!("{}", type_name::<T>());
        assert!(!debug_string.is_empty());
    }

    #[test_case_source(JsonFile("tests/test_ddt_data.json"))]
    fn test_users_inferred(user: User) {
        assert!(user.age > 0);
    }

    #[test_case_source(JsonFile("tests/test_vec_of_vec.json"))]
    fn test_vec_of_vec(v: Vec<u32>) {
        assert!(!v.is_empty());
    }

    #[test_case_source(JsonFile("tests/test_single_vec.json"))]
    fn test_single_vec(v: Vec<u32>) {
        assert!(!v.is_empty());
    }
}
