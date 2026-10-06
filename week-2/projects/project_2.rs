fn main() {
	let toshiba: f64 = 450000.0;
	let mac: f64 = 1500000.0;
	let hp: f64 = 750000.0;
	let dell: f64 = 2850000.0;
	let acer: f64 = 250000.0;

	// Total sum of the amounts
	let sum = toshiba + mac + hp + dell + acer;

	// Average of the 5 items
	let average = sum / 5.0;
	println!("The total sum of the amounts is {}", sum);
	println!("The average of the amounts is {}", average);
}	
