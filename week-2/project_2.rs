fn main(){
	let toshiba: f64 = 450_000.00;
	let mac: f64 = 1_500_000.00;
	let hp: f64 = 750_000.00;
	let dell: f64 = 2_850_000.00;
	let acer: f64 = 250_000.00;
	
	let sum: f64 = toshiba + mac + hp + dell + acer;

	let average: f64 = sum / 5.0;

	println!("The sum of the sales is: {}", sum);
	println!("The average of the  sales is {}", average);
}