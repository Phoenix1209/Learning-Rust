pub enum Post {
	Draft(String),
	PendingReview(String),
	Published(String)
}

impl Post {
	pub fn new() -> Post {
		Post::Draft(String::new())
	}

	pub fn add_text(&mut self, text: &str) {
		if let Post::Draft(content) = self {
			content.push_str(text)
		}
	}

	pub fn request_review(self) -> Post {
		match self {
			Post::Draft(content) => Post::PendingReview(content),
			other => other
		}
	}

	pub fn approve(self) -> Post {
		match self {
			Post::PendingReview(content) => Post::Published(content),
			other => other
		}
	}

	pub fn content(&self) -> &str {
		match self {
			Post::Published(content) => content,
			_ => ""
		}
	}
}