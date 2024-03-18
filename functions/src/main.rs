fn main() {
	println!("Hello, World");
	another_function(5, 'h');

	// expression
	let y = {
		let x = 3;
		x + 1
	};

	println!("The value of y is {y}");
	
	let x = five();
	println!("The value of x is {x}");
}

// function expression
fn five() -> u32 {
	5
}

fn another_function(x: i32, unit_label: char) {
	println!("Another function");
	println!("Argument value is {x}{unit_label}");	
}
