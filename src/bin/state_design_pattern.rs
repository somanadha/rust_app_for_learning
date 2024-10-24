fn main() {
    let mut post: Post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approved(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

pub struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approved(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

pub struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approved(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

pub struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approved(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

impl Post {
    pub fn new() -> Self {
        Post {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        // if let Some(state) = self.state.take() {
        //     self.state = Some(state.request_review());
        // }

        match self.state.take() {
            Some(state) => self.state = Some(state.request_review()),
            None => (),
        }
    }
    pub fn approve(&mut self) {
        // if let Some(state) = self.state.take() {
        //     self.state = Some(state.approved());
        // }
        match self.state.take() {
            Some(state) => self.state = Some(state.approved()),
            None => (),
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}
