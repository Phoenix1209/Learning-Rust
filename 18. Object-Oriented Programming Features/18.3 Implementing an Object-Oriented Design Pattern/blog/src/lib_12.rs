// This code does not compile!

pub struct Post {
	state: Option<Box<dyn State>>,
	content: String
}

impl Post {
	pub fn new() -> Post {
		Post {
			state: Some(Box::new(Draft {})),
			content: String::new()
		}
	}

	pub fn add_text(&mut self, text: &str) {
		// Is State who decide WHAT text to add; Post is the one to modify it.
		let allowed = self.state.as_ref().unwrap().add_text(text);
		self.content.push_str(allowed)
	}

	pub fn content(&self) -> &str {
		self.state.as_ref().unwrap().content(self)
	}

	pub fn request_review(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.request_review())
		}
	}

	pub fn reject(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.reject())
		}
	}

	pub fn approve(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.approve())
		}
	}
}

trait State {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		self
	}

	fn approve(self: Box<Self>) -> Box<dyn State> {
		self
	}

	fn reject(self: Box<Self>) -> Box<dyn State> {
		self
	}

	// Default Implementations: Only the states that need it rewrite it.
	fn content<'a>(&self, _post: &'a Post) -> &'a str {
		""
	}

	fn add_text<'a>(&self, _text: &'a str) -> &'a str {
		""
	}
}

struct Draft {}

impl State for Draft {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		Box::new(PendingReview { approvals: 0 })
	}

	fn add_text<'a>(&self, text: &'a str) -> &'a str {
		text
	}
}

struct PendingReview {
	approvals: u8
}

impl State for PendingReview {
	fn approve(self: Box<Self>) -> Box<dyn State> {
		let approvals = self.approvals + 1;

		if approvals >= 2 {
			Box::new(Published {})
		} else {
			Box::new(PendingReview { approvals })
		}
	}

	fn reject(self: Box<Self>) -> Box<dyn State> {
		Box::new(Draft {})
	}
}

struct Published {}

impl State for Published {
	fn content<'a>(&self, post: &'a Post) -> &'a str {
		&post.content
	}
}

/*
Another downside is that we’ve duplicated some logic. To eliminate some of the duplication, we
might try to make default implementations for the request_review and approve methods on the
State trait that return self. However, this wouldn’t work: When using State as a trait object, the
trait doesn’t know what the concrete self will be exactly, so the return type isn’t known at compile
time.
*/