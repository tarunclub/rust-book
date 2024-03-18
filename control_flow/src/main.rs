fn main() {
    	let number = 3;
	let condition = true;
	
	if number < 5 {
		println!("Condition is true");
	} else {
		println!("condition is false");
	}
	
	let number = if condition { 5 } else { 6 };
	println!("{number}");

	let mut counter = 0;

	let result = loop {
		counter += 1;

		if counter >= 20 {
			break counter * 2
		}
	};

	println!("The value of result is {result}");
	
	// while loop
	let mut num = 3;
	while num != 0 {
		println!("{num}");

		num -= 1;
	}
	
	println!("LIFTOFF");
	
	// for loop
	let b = [10, 20, 30, 40, 50, 60];
	
	for elem in b {
		println!("the value is: {elem}");
	}
}
