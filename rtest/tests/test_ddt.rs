use rtest::test_case_source;

mod my_module {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct User {
        pub name: String,
        pub age: u32,
    }
}


#[cfg(test)]
mod tests {
    use crate::my_module::User;
    use super::*;

    #[test_case_source("tests/test_ddt_data.json", User)]
    fn test_users(user: User) {
        println!("User age: {}", user.age);
        assert!(user.age > 0);
    }

}
