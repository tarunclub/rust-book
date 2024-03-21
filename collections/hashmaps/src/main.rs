use std::collections::HashMap;

fn main() {
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	let team_name = String::from("Blue");
	let score = scores.get(&team_name).copied().unwrap_or(0);

	println!("{score}");

	for (key, values) in &scores {
		println!("{key}: {values}");
	}

	let first = String::from("Tarun Kumar");
	let second = String::from("Shinchan Nohara");

	let mut names = HashMap::new();

	names.insert(first, second);

	for (key, value) in &names {
		println!("{key}: {value}");
	}
}
