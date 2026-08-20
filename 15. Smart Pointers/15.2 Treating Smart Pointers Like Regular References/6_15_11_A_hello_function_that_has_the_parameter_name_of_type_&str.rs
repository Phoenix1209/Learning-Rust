fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
	let name = String::from("Rust");
	hello(&name);
}