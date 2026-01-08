use rust_test_framework::test_params_source;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
}

// Returns a single object, but we expect a Vec.
// This should be a compile error if we want to satisfy the issue description.
#[test_params_source(JsonResponse::<Vec<Post>>("https://jsonplaceholder.typicode.com/posts/1"))]
fn test_type_mismatch(posts: Vec<Post>) {
    assert!(!posts.is_empty());
}

fn main() {}
