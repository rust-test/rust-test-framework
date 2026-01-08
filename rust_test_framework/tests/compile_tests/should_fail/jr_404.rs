use rust_test_framework::test_params_source;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
}

#[test_params_source(JsonResponse("https://jsonplaceholder.typicode.com/invalid-path-that-returns-404"))]
fn test_404(post: Post) {
    assert!(post.id > 0);
}

fn main() {}
