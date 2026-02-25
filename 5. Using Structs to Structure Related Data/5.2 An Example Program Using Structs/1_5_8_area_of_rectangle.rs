// Listing 5-8: Calculating the area of a rectangle specified by separate width and height variables

fn main() {
	let width1 = 30;
	let height1 = 50;

	println!(
		"The area of the rectangle is {} square pixels.",
		area(width1, height1)
	);
}

fn area(width: u32, height: u32) -> u32 {
	width * height
}