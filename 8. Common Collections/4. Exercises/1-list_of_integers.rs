/* Given a list of integers, use a vector and return the median (when sorted, the value in the
middle position) and mode (the value that occurs most often; a hash map will be helpful here)
of the list. */

fn main() {
    use std::collections::HashMap;

	let mut v = vec![1, 2, 3, 4, 4, 5, 6, 4, 2, 8, 7, 4];
	// let mut v = vec![];
	println!("Vector: {v:?}");

	v.sort();
	println!("Sorted vector: {v:?}");

	let median = get_median(&v);
	println!("Median: {median}");

	let mut mode = HashMap::new();
	let mut max = 0;
	let mut max_key = 0;
	let mut max_value = 0;

	for number in &v {
		let count = mode.entry(number).or_insert(0);
		*count += 1;

		if *count > max {
			max_key = *number;
			max_value = *count;
			max = *count;
		}

	}
	println!("Mode: {max_key} with {max_value} occurrences")
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

	match median {
		Some(median) => return *median,
        None => {
			return *median.unwrap_or(&0)
		}
	}
}