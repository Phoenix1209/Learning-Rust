// No compila
fn main() {
	let x: i8 = 5;
	let y: Option<i8> = Some(5);

	let sum = x + y;
	// let x = x + y;
	// i8 y Option<i8> son tipos diferentes de datos

	println!("{}",x);
	println!("{:?}",y);
	println!("{:?}",sum);
}