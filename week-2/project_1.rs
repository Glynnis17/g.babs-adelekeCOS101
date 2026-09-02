fn main() {
	let p:f64 = 520_000_000.00; // p = Principle Amount
	let r:f64 = 0.1; // r = Rate
	let n:f64 = 1.0; // n = no. of times compounded
	let t:f64 = 5.0; // t = no. of years/time

	//The formula for Compound Interest is as follows
	let a = p * (1.0 + (r/n)).powf(n * t);
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound Interest is {}", ci);
}