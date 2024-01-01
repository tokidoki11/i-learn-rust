use super::Draft;
use super::Post;
use super::State;
pub struct Accepted {}

impl State for Accepted {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
