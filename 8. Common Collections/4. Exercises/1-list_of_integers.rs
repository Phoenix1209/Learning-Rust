/* Given a list of integers, use a vector and return the median (when sorted, the value in the
middle position) and mode (the value that occurs most often; a hash map will be helpful here)
of the list. */

fn main() {
    use std::collections::HashMap;

	let v = vec![1, 2, 3, 8, 4, 5, 6, 7, 8, 8, 7, 7];
	println!("Vector: {v:?}");

	let mut v2 = v.clone();
	v2.sort();
	println!("Sorted vector: {v2:?}");

	let median = get_median(&v2);
	println!("Median: {median}\n");

	let mut mode = HashMap::new();
	let mut max: i32 = v[0];
	let mut max_key: i32 = v2[0];
	let mut max_value: i32 = v2[0];

	let mut i: u16 = 1;
	for number in &v2 {
		let count = mode.entry(number).or_insert(0);
        *count += 1;

		if *count > max {
			max_key = *number;
			max_value = *count;
			max = *count;
		}
		
		i += 1;

		println!("number: {number}");
		println!("HashMap: {mode:?}");
	}

	println!("\nHashMap: {mode:?}");
	println!("Mode: {max_key} with {max_value} occurrences");
}

fn get_median(v: &Vec<i32>) -> i32 {
	let mut median = None;

	match v.is_empty(){
		true => println!("Vector is empty."),
		false => {
			if v.len() % 2 == 0 {
				median = v.get(v.len() / 2 - 1);
			} else {
				median = v.get(v.len() / 2);
			}
		}
	}

	return *median.unwrap_or(&0)
}