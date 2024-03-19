#[derive(Debug)]
enum IpAddrKind {
	V4(String),
	V6(String),
}

//#[derive(Debug)]
//struct IpAddr {
//	kind: IpAddrKind(String::from("::1")),
//	address: String,
//}

fn main(){
 	let home = IpAddrKind::V4(String::from("127.0.0.1"));
	let loopback = IpAddrKind::V6(String::from("::1"));

	let x: i8 = 5;
	let y: Option<i8> = Some(5);

	let sum = x + y.unwrap_or(0);

	println!("{sum}");
}

