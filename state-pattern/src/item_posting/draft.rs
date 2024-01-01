pub struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}
