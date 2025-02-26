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

pub fn notify_multiple<T: Summary + Display>(item1: &T, item2: &T) {
    println!("News 1: {}", item1);
    println!("News 2: {}", item2);
}

fn main() {
    let article1 = NewsArticle {
        headline: String::from("Rust 2025 llega con novedades"),
        author: String::from("Alice"),
    };

    let article2 = NewsArticle {
        headline: String::from("Nuevo compilador optimizado"),
        author: String::from("Bob"),
    };

    notify_multiple(&article1, &article2); // ✅ Ambos son NewsArticle
}