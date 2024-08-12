/* Convert strings to pig latin. The first consonant of each word is moved to the end of
the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay
added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8
encoding! */

fn main() {
	// use Writte for flush()
	use std::io::{stdin, stdout, Write};

	let mut word = String::new();

	print!("Enter a latin word: ");
    let _=stdout().flush();
	stdin().read_line(&mut word).expect("Failed to read line.");

	// let mut word = "Зд".to_string();
    if let Some('\n')=word.chars().next_back() {
        word.pop();
    }
	if let Some('\r')=word.chars().next_back() {
        word.pop();
    }
	//println!("{}", word.len());
	if word.len() == 0 {
		println!("Exit code 1: No word inserted.");
		std::process::exit(1)
	}
	// println!("You insert {word}");

	let letter = word.chars().next().unwrap();
	word.push('-');

	if letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u' {
		word.push_str("hay")
	} else {
		word.remove(0);
		word.push_str(&letter.to_string());
		word.push_str("ay");
	}
	println!("{word}");
}

// Зд