use std::io;

fn main() {
    println!("--- Employee Incentive Calculator ---");

    // 1. Ask if the employee is experienced
    println!("Is the employee experienced? (yes/no):");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read line");
    let is_experienced = experience_input.trim().to_lowercase();

    // 2. Process logic based on experience
    if is_experienced == "yes" {
        // Ask for age if they are experienced
        println!("Enter the employee's age:");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read line");
        
        // Convert the age text into a whole number (integer)
        let age: u32 = age_input.trim().parse().expect("Please enter a valid age number");

        // 3. Check age criteria for experienced employees
        if age >= 40 {
            println!("Annual Incentive: ₦1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual Incentive: ₦1,480,000");
        } else if age < 28 {
            println!("Annual Incentive: ₦1,300,000");
        } else {
            // Handles missing ages (28 and 29) from the project sheet guidelines
            println!("Annual Incentive: ₦1,300,000 (Default for ages 28-29)");
        }
         } else {
        // 4. Not experienced criteria
        println!("Annual Incentive: ₦100,000");
    }
}
