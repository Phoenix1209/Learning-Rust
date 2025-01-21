fn main() {
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];

	for cell in &row {
		match cell {
			SpreadsheetCell::Int(value) => println!("Int: {}", value),
			SpreadsheetCell::Float(value) => println!("Float: {}", value),
			SpreadsheetCell::Text(value) => println!("Text: {}", value),
		}
	}
}