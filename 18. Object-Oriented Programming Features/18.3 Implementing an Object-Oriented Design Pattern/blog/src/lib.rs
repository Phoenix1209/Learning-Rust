pub struct DraftPost {
	content: String
}

pub struct PendingReviewPost {
	content: String
}

pub struct ApprovedOncePost {
	content: String
}

pub struct Post {
	content: String
}

impl DraftPost {
	pub fn add_text(&mut self, text: &str) {
		self.content.push_str(text)
	}

	pub fn request_review(self) -> PendingReviewPost {
		PendingReviewPost {
			content: self.content
		}
	}
}

impl PendingReviewPost {
	// First approve: isn't a published Post yet.
	pub fn approve(self) -> ApprovedOncePost {
		ApprovedOncePost {
			content: self.content
		}
	}

	pub fn reject(self) -> DraftPost {
		DraftPost {
			content: self.content
		}
	}
}

impl ApprovedOncePost {
	// Second approve: now it is published
	pub fn approve(self) -> Post {
		Post {
			content: self.content
		}
	}

	pub fn reject(self) -> DraftPost {
		DraftPost {
			content: self.content
		}
	}
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

	// pub fn reject(self) -> PendingReviewPost {
	// 	PendingReviewPost {
	// 		content: self.content
	// 	}
	// }
}