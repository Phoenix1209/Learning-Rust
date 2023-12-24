fn main() {
	let some_number = Some(5);
	// let some_number = Some<i32>; <- El Some siempre va con parentesis
	// let some_number = Some(i32); <- No sirve
	// let some_number = Some(T);	// <- No sirve
	// let some_number = Option<i32>;	// <- Esto no sirve
	// let some_number = Option<i32> = 5;	// <- Ni esto
	let some_char = Some('e');

	let absent_number: Option<i32> = None;
	// let absent_number: Option<i32>;	// <- hay que especificar que lleva none sino no sirve
	// let absent_number: Option<i32> = Some(T);	// <- Esto tampoco sirve
	// let absent_number: Option<i32> = Some(5);
	/*	La anterior función sí funciona pero es mejor quitar el option y poner directamente
		el some como se encuentra más arriba */

	println!("some_number: {:?}",some_number);
	println!("some_char: {:?}",some_char);
	println!("absent_number: {:?}",absent_number)
}