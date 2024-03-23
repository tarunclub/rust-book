fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
	let mut largest = &list[0];

	for item in list {
		if item > largest {
			largest = item;
		}
	}

	largest
}

struct Point<T, U> {
	x: T,
	y: U,
}

fn main() {
	let number_list = vec![10, 20, 30, 40, 50, 60, 150, 5];

	let result1 = largest(&number_list);
	println!("The largest number is {result1}");

	let p1 = Point {
		x: 5, 
		y: 1.2,
	};

	let p2 = Point {
		x: 2.4,
		y: 1,
	};
}
