use super::Post;
pub trait State {
    //note that rather than having self, &self, or &mut self as the first parameter of the method,
    //we have self: Box<Self>. This syntax means the method is only valid when called on a Box holding the type. This syntax takes ownership of Box<Self>,
    //invalidating the old state so the state value of the Post can transform into a new state
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}
