// Error happen in program execute, not in compile time

fn main() {
    let v = vec![1, 2, 3, 4, 5];

	// ¡PANIC!
    // let does_not_exist = &v[100];
	// println!("The element is: {does_not_exist}");

    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(does_not_exist) => println!("The third element is {does_not_exist}"),
        None => println!("There is no third element."),
    }
}