fn print_type<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main(){
    let hello = "Здравствуйте";
	// The next line will always get error at compile time
    // let answer = &hello[0];
    let answer = &hello.chars();
	let asdf = vec![hello];
	let letter = &asdf[0];
	/* The next line is what I was locking for but is unstable because UTF-8 does not define what "character"
	is. In this case, chars are Unicode scalar values, and so the first	char of a &str is going to
	be between one and four bytes. */
	let chars: Vec<char> = hello.chars().collect();
	let ch = chars[0];
	let ch2 = hello.chars().next().unwrap();
	let ch3 = hello.chars().nth(1).unwrap();

	println!("hello: {hello}");
	print_type(hello);
	println!("answer: {answer:?}");
	print_type(answer);
	println!("asdf: {asdf:?}");
	print_type(&asdf);
	println!("letter: {letter:?}");
	print_type(letter);
	println!("chars: {chars:?}");
	print_type(&chars);
	println!("ch: {ch}");
	print_type(&ch);
	println!("ch2: {ch2}");
	print_type(&ch2);
	println!("ch3: {ch3}");
	print_type(&ch3);
}