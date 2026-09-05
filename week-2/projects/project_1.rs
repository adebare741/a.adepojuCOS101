fn main() {
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let t:f64 = 5.0;

	// simple interest
	let si = (p*r*t) / 100.0;
	println!("simple interest is {}", si);
}	