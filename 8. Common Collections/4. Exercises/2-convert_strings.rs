/* Convert strings to pig latin. The first consonant of each word is moved to the end of
the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay
added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8
encoding! */

fn main() {
	use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Please enter some text: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    println!("You typed: {}",s);
}