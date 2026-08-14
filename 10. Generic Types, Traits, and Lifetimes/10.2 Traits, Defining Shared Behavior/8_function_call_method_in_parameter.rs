pub trait Summary {
	fn summarize(&self) -> String;
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
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

pub fn notify(item: &impl Summary) {
	println!("Breaking news! {}", item.summarize());
}

fn main() {
	let article = NewsArticle {
		headline: String::from("Rust avanza en popularidad"),
		location: String::from("Internet"),
		author: String::from("Jane Doe"),
		content: String::from("Rust está siendo adoptado por más desarrolladores debido a su seguridad y rendimiento."),
	};

	let tweet = Tweet {
		username: String::from("rustacean"),
		content: String::from("¡Rust es increíble!"),
		reply: false,
		retweet: false,
	};

	notify(&article);
	notify(&tweet);
}