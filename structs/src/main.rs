#[derive(Debug)]
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	is_active: bool,
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

impl Rectangle {
	fn square(size: u32) -> Rectangle {
		 Rectangle {
			width: size,
			height: size
		}
	}
}
fn main() {
	let mut user1 = User {
		email: String::from("tarunk1004@gmail.com"),
		username: String::from("Tarun"),
		sign_in_count: 1,
		is_active: true,
	};

	user1.username = String::from("tarunkm");
	let name = user1.username;
	println!("{name}");

	let user2 = User {
		email: String::from("user2@gmail.com"),
		username: String::from("user2"),
		..user1
	};

	let user2_email = user2.email;
	println!("{user2_email}");
	
	// tuple structs
	struct Color(i32, i32, i32);

	struct Point(i32, i32, i32);

	let rect = Rectangle {
		width: 30,
		height: 20,
	};

	println!("{}", rect.area());

	println!("{:#?}", rect);
}
