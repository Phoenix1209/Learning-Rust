// This code will not compile because need a main body

pub fn notify<T: Summary>(item: &T) {
	println!("Breaking news! {}", item.summarize());
}