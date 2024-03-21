fn main() {
	let mut str = String::new();

	let s = String::from("initial contents");

	println!("{s}");

	str.push_str("Namaste");
	println!("{str}");

	// push a single char	
	let mut news = String::from("lo");
	news.push('i');

	println!("{news}");

	let s1 = String::from("Hello,");
	let s2 = String::from("World");
	let s3 = s1 + &s2;

	println!("{s3}");

//	let h = &s1[..2];

//	println!("{h}");
	
	for c in "Hello, World".chars() {
		println!("{c}");
	}
	
}
