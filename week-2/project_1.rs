fn main(){
	let p: f64 = 520_000_000.0;
	let r: f64 = 10.0;
	let factor = 1.0 + (r/100.0);

	
	let a = p * factor * factor * factor * factor * factor; 
    println!("Amount is {}",a);
    let ci = a - p;
    println!("The compound interest (CI) is {}",ci);
}