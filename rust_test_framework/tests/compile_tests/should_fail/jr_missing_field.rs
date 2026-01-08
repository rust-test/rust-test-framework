use rust_test_framework::test_params_source;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    username: String,
}

#[test_params_source(JsonResponse("https://jsonplaceholder.typicode.com/posts/1"))]
fn test_json_response_with_missing_field(user: User) {
    assert!(!user.username.is_empty());
}

fn main() {}
