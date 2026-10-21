// Listing 18-15: Implementing request_review methods on Post and the State trait

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // --snip--
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
        ""
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

/*
Note that rather than having self, &self, or
&mut self as the first parameter of the method, we have self: Box<Self>. This syntax means the
method is only valid when called on a Box holding the type. This syntax takes ownership of
Box<Self>, invalidating the old state so that the state value of the Post can transform into a new
state.

To consume the old state, the request_review method needs to take ownership of the state value.
This is where the Option in the state field of Post comes in: We call the take method to take the
Some value out of the state field and leave a None in its place because Rust doesn’t let us have
unpopulated fields in structs. This lets us move the state value out of Post rather than borrowing
it. Then, we’ll set the post’s state value to the result of this operation.
*/