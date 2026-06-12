// Listing 8-21: Accessing the score for the Blue team stored in the hash map

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{scores:?}");

	println!("\nteam_name: {team_name}");
	println!("score: {score}")
}