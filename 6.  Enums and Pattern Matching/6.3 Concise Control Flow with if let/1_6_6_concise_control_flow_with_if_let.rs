// Listing 6-6: A match that only cares about executing code when the value is Some

fn main() {
	let config_max = Some(3u8);
	match config_max {
		Some(max) => println!("The maximum is configured to be {}", max),
		_ => (),
	}
}