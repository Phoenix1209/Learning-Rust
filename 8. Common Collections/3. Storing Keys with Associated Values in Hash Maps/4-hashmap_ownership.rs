fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
	// map took ownership of field_name and field_value
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

	println!("{field_name}")
}