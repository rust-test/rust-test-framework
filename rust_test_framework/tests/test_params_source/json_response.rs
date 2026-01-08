use rust_test_framework::test_params_source;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
}

#[test_params_source(JsonResponse("https://jsonplaceholder.typicode.com/posts/1"))]
fn test_json_response_single_object(post: Post) {
    assert!(!post.title.is_empty());
}

#[test_params_source(JsonResponse("https://jsonplaceholder.typicode.com/posts"))]
fn test_json_response_list(post: Post) {
    assert!(!post.title.is_empty());
}

#[test_params_source(JsonResponse::<Vec<Post>>("https://jsonplaceholder.typicode.com/posts?_limit=3"))]
fn test_json_response_list_explicit_type(posts: Vec<Post>) {
    assert_eq!(posts.len(), 3);
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    username: String,
}

#[test_params_source(JsonResponse("https://jsonplaceholder.typicode.com/users/1"))]
fn test_json_response_user(user: User) {
    assert!(!user.username.is_empty());
}
