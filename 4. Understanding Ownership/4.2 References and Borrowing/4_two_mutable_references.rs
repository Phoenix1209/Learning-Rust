// No compila porque solo se puede traspasar con r1, ya con r2 marca error

fn main() {
	let mut s = String::from("hello");

	let r1 = &mut s;
	let r2 = &mut s;

    println!("{}, {}", r1, r2);
}