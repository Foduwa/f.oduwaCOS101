use std::io;

fn main() {
    println!("Enter values for a, b, and c:");

    // 1. Read input for 'a'
    println!("Enter a: ");
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("Failed to read line");
    let a: f64 = input_a.trim().parse().expect("Please enter a valid number");

    // 2. Read input for 'b'
    println!("Enter b: ");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("Failed to read line");
    let b: f64 = input_b.trim().parse().expect("Please enter a valid number");

    // 3. Read input for 'c'
    println!("Enter c: ");
    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).expect("Failed to read line");
    let c: f64 = input_c.trim().parse().expect("Please enter a valid number");

    // 4. Calculate the discriminant
    let d = b * b - 4.0 * a * c;
    println!("Discriminant (d) = {}", d);

    // 5. Determine the roots based on 'd'
    if d > 0.0 {
        // Two distinct real roots
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct roots: x1 = {}, x2 = {}", root1, root2);
    } else if d == 0.0 {
        // Exactly one real root
        let root = -b / (2.0 * a);
        println!("Exactly one real root: x = {}", root);
    } else {
        // No real roots
        println!("No real roots (the roots are complex numbers).");
    }
}