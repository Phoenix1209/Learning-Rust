// emphasis on Option. It can return Some or None.
fn last_char_of_first_line(text: &str) -> Option<char> {
	text.lines().next()?.chars().last()
}

fn main() {
	assert_eq!(
		last_char_of_first_line("Hello, world\nHow are you today?"),
		Some('d')
	);

	println!("{:?}", assert_eq!(last_char_of_first_line(""), None));
	println!("{:?}", assert_eq!(last_char_of_first_line("\nhi"), None));
}

/*
This function returns Option<char> because it’s possible that there is a character there,
but it’s also possible that there isn’t. This code takes the text string slice argument
and calls the lines method on it, which returns an iterator over the lines in the string.
Because this function wants to examine the first line, it calls next on the iterator to get
the first value from the iterator. If text is the empty string, this call to next will return
None, in which case we use ? to stop and return None from last_char_of_first_line. If text
is not the empty string, next will return a Some value containing a string slice of the first line in text.

The ? extracts the string slice, and we can call chars on that string slice to get an iterator
of its characters. We’re interested in the last character in this first line, so we call last
to return the last item in the iterator. This is an Option because it’s possible that the first
line is the empty string; for example, if text starts with a blank line but has characters on
other lines, as in "\nhi". However, if there is a last character on the first line, it will
be returned in the Some variant. The ? operator in the middle gives us a concise way to express
this logic, allowing us to implement the function in one line. If we couldn’t use the ? operator
on Option, we’d have to implement this logic using more method calls or a match expression.

Note that you can use the ? operator on a Result in a function that returns Result, and you can
use the ? operator on an Option in a function that returns Option, but you can’t mix and match.
The ? operator won’t automatically convert a Result to an Option or vice versa; in those cases,
you can use methods like the ok method on Result or the ok_or method on Option to do the conversion explicitly.

So far, all the main functions we’ve used return (). The main function is special because it’s the
entry point and exit point of an executable program, and there are restrictions on what its return
type can be for the program to behave as expected.
*/