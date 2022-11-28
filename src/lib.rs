// #[derive(Debug)]
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
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
}
#[allow(unused)]
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

mod tests {
    use crate::Post;

    #[test]
    fn cant_view_draft_post_content() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");

        assert_eq!("",post.content());
    }

    #[test]
    fn cant_update_post_without_approval() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        post.request_review();
        // post.approve();

        assert_eq!("",post.content())
    }

    #[test]
    fn cant_approve_without_requesting_review() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        // post.request_review();
        post.approve();

        assert_eq!("",post.content())
    }

    #[test]
    fn post_updates_after_review_and_approval() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        post.request_review();
        post.approve();

        assert_eq!("I ate a salad for lunch today",post.content())
    }
}