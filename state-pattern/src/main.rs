mod item_posting;
fn main() {
    let mut post = item_posting::Post::new();
    post.set_content(String::from("I ate a salad for lunch today"));
    println!("post.content = {}", post.content());
    post.request_review();
    println!("post.content = {}", post.content());
    post.approve();
    println!("post.content = {}", post.content());
    post.reject();
    println!("post.content = {}", post.content());
}
