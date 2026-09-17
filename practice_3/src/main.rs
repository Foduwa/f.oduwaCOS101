// Rust program to calculate the area of a 
// triangle for a given base and height 

use std::io;

fn main()
{
    let mut side1 = String::new();
    let mut side2 = String::new();

    println!("Enter base of triangle:");
    io::stdin().read_line(&mut side1).expect("Not a valid string");
    let base:f32 = side1.trim().parse().expect("Nota  valid number");

    println!("Enter height of triangle: ");
    io::stdin().read_line(&mut side2).expect("Not a valid string");
    let height:f32 = side2.trim().parse().expect("Not a valid number");

    if base > 0.0 {
        let area:f32 =(base * height) / 2.0;
        println!("Area of triangle: {}", area);
    }
}







    
