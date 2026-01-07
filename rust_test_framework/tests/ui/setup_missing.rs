use rust_test_framework::test_fixture;

#[test_fixture]
mod tests {
    #[setup]
    fn my_setup() {}

    #[test]
    fn test_dummy() {}
}

fn main() {}
