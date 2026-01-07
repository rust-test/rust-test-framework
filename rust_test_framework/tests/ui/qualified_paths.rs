use rust_test_framework::test_fixture;

#[test_fixture]
mod tests {
    #[rust_test_framework::setup]
    fn my_setup() {}

    #[rust_test_framework::teardown]
    fn my_teardown() {}

    #[test]
    fn test_dummy() {}
}

fn main() {}
