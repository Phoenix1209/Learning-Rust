pub trait Summary {
	fn summarize(&self) -> String;
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

// Especificamos el tipo concreto `Tweet`
fn returns_summarizable() -> Tweet {
	Tweet {
		username: String::from("horse_ebooks"),
		content: String::from(
			"of course, as you probably already know, people",
		),
		reply: false,
		retweet: false,
	}
}

fn main() {
	let tweet = returns_summarizable();
	println!("Generated Tweet: {}", tweet.summarize());
}