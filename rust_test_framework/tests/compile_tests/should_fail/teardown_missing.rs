use rust_test_framework::test_fixture;

#[test_fixture]
mod tests {
    #[teardown]
    fn my_teardown() {}

    #[test]
    fn test_dummy() {}
}

fn main() {}
