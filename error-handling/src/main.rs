use std::fs::File;

fn main() {
//	panic!("crash and burn");

	let greeting_file_result = File::open("hello.txt").unwrap();

//	let greeting_file = match greeting_file_result {
//		Ok(file) => file,
//		Err(error) => panic!("Problem opening the file: {:?}", error),
//	};

	
}
