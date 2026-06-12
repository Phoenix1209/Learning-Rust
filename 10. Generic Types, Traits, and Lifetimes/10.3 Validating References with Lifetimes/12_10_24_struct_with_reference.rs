// Listing 10-24: A struct that holds a reference, requiring a lifetime annotation

#[derive(Debug)]

struct ImportantExcerpt<'a> {
	part: &'a str,
}

fn main() {
	let novel = String::from("Call me Ishmael. Some years ago...");
	println!("{novel}");
	let first_sentence = novel.split('.').next().unwrap();
	println!("{first_sentence}");
	let i = ImportantExcerpt {
		part: first_sentence,
	};
	//println!("{:?}", i)
	println!("{:#?}", i)
}