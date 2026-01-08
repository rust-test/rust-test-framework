use rust_test_framework::test_params_source;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
}

// Google returns HTML, which is invalid JSON
#[test_params_source(JsonResponse("https://www.google.com"))]
fn test_invalid_json(post: Post) {
    assert!(post.id > 0);
}

fn main() {}
