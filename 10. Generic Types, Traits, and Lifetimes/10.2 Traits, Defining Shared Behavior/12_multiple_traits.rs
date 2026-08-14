use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

// Implementamos Display para que también cumpla el otro trait
impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.headline, self.author)
    }
}

pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item);
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust avanza en popularidad"),
        author: String::from("Jane Doe"),
    };

    notify(&article); // ✅ Funciona porque NewsArticle implementa Summary y Display
}