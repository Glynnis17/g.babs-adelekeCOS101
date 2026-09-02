fn main() {
	let toshiba:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00;
	let dell:f64 = 2_850_000.00;
	let acer:f64 = 250_000.00;
	let sum = toshiba + mac + hp + dell + acer;

	println!("The sum of the sales expenses is: #{:.2}", sum);

	let toshiba_qty:f64 = 2.0;
	let mac_qty:f64 = 1.0;
	let hp_qty:f64 = 3.0;
	let dell_qty:f64 = 3.0;
	let acer_qty:f64 = 1.0;
	let total_qty:f64 = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty;

	let average = sum / total_qty;
	println!("The average of the sales expenses is: #{:.2}", average);
}