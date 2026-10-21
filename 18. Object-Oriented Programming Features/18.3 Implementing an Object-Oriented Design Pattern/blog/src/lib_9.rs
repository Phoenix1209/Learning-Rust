pub struct Post {
	state: State,
	content: String
}

enum State {
	Draft,
	PendingReview,
	Published
}

impl Post {
	pub fn new() -> Post {
		Post {
			state: State::Draft,
			content: String::new()
		}
	}

	pub fn add_text(&mut self, text: &str) {
		self.content.push_str(text)
	}

	pub fn content(&self) -> &str {
		match self.state {
			State::Published => &self.content,
			_ => ""
		}
	}

	pub fn request_review(&mut self) {
		if matches!(self.state, State::Draft) {
			self.state = State::PendingReview
		}
	}

	pub fn approve(&mut self) {
		if matches!(self.state, State::PendingReview) {
			self.state = State::Published
		}
	}
}