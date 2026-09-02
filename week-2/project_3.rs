fn main() {
	let p:f64 = 210_000.00; // p = principle amount
	let r:f64 = 5.0; // r = rate
	let n:f64 = 3.0; // n = no. of years

	let a = p * (1.0 - (r/100.00)).powf(n);
	println!("The depreciated value of the TV is: {:.2}", a);
	
}