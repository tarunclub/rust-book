fn main() {
	let mut x = 5;
	println!("The value of x is: {x}");
	x = 6;
	println!("The value of x is: {x}");

	let tup: (i32, f64, u8) = (500, 6.4, 1);

	let (x, y, z) = tup;
	
	println!("{x}, {y}, {z}");
	
	println!("tup.x");

	let arr = [1, 2, 3, 4, 5, 6];
	
	println!("{:#?}", arr);

	let a: [i32; 5] = [1, 2, 3, 4, 5];
	
	let first = a[0];
	let second: i32 = a[1];
	
	println!("{first}, {second}");
	
	
}
