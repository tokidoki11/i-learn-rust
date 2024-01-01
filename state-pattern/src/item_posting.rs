mod accepted;
mod draft;
mod pending_review;
mod state;

use accepted::Accepted;
use draft::Draft;
use pending_review::PendingReview;
use state::State;

pub struct Post {
    //we call the take method to take the Some value out of the state field and leave a None in its place, because Rust doesn’t let us have unpopulated fields in structs. This lets us move the state value out of Post rather than borrowing it.
    //Then we’ll set the post’s state value to the result of this operation.
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::from(""),
        }
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}
