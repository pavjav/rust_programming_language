/// The state pattern is about using the same method to implement different logic depending on the state
/// of the object. Below is an example of a Post struct with an optional pointer to objects that implement
/// the State trait
/// We use the State trait to implement request_review, approve, and content() methods
/// Draft:
///     request_review() -> returns a PendingReview struct
///     approve() -> do nothing
///     content() -> return ""
/// PendingReview:
///     request_review() -> do nothing
///     approve() -> returns a Publushed Struct
///     content() -> return ""
/// Published:
///     request_review() -> do nothing
///     approve() -> do nothing
///     content() -> return actual content of a post

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn approve(&mut self) {
        // Can use if let expression
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
        // Or can use match assignment
        self.state = match self.state.take() {
            Some(s) => Some(s.approve()),
            None => None
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        assert_eq!(post.content(),"");
        post.request_review();
        assert_eq!(post.content(),"");
        post.approve();
        assert_eq!(post.content(),"I ate a salad for lunch today");
    }
}
