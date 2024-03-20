fn main() {
	let mut v: Vec<i32> = Vec::new();

	let v1 = vec![1, 2, 3];

	println!("{:#?}", v1);
	
	v.push(4);
	v.push(5);
	v.push(6);

	println!("{:#?}", v);
	
	let third: &i32 = &v[2];
	println!("The third element is {third}");

	let third: Option<&i32> = v1.get(2);
	match third {
		Some(third) => println!("The third element is {third}"),
		None => println!("There is no third element"),
	}

	let last: Option<&i32> = v1.get(100);
	
	match last {
		Some(last) => println!("The last element is {last}"),
		None => println!("There is no such element"),
	}
	
	let mut vec = vec![100, 32, 57];
	for n_ref in &v {
		let n_plus_one: i32 = *n_ref + 1;
		println!("{n_plus_one}");
	}

	for n_ref in &mut vec {
		*n_ref += 1;
		println!("{n_ref}");
	}
}	
