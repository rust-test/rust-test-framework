use rust_test_framework::test_fixture;

#[test_fixture]
mod tests {
    use rust_test_framework::{setup, teardown};

    #[setup]
    fn my_setup() {}

    #[teardown]
    fn my_teardown() {}

    #[test]
    fn test_dummy() {}
}

fn main() {}
