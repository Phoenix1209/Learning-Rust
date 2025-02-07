/*
We use expect in the same way as unwrap: to return the file handle or call the panic! macro.
The error message used by expect in its call to panic! will be the parameter that we pass to expect,
rather than the default panic! message that unwrap uses.
*/

use std::fs::File;

fn main() {
	let greeting_file = File::open("hello.txt")
		.expect("hello.txt should be included in this project");
}