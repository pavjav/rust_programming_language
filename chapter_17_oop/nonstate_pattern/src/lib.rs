/// The state pattern tradeoff is that we must impement the state transitions on the states themselves
/// We must also duplicate a lot of logic. We can instead encode the states directly into the structs
/// and consider trait transitions as transformations into new types

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

pub struct PendingReviewPost {
    content: String
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        let post = post.approve();
        assert_eq!(post.content(),"I ate a salad for lunch today");
    }
}