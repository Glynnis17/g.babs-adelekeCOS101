fn main() {
	let p:f64 = 1000.00; //p stands for principle
	let r:f64 = 1.0; //r stands for rate
	let t:f64 = 2.0; //t stands for time

	//the following is to calculate simple interest
	let a = p * (1.0 + (r/100.00)) * t;
	println!("Amount is {}", a);
	let si = a - p;
	println!("Simple Interest is {}", si);
}