fn main() {
    
    let aps_levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    
   // from ranges of 0 ,1 and 2 excluding the 3 
    let entry_level_slice = &aps_levels[0..3];
    let employee = ("Lawyer", "Associate");

    

    if employee.0 == "Lawyer" {
    if employee.1 == "Paralegal" {
            println!("Level: {}", aps_levels[0]);
        }
    if employee.1 == "Junior Associate" {
            println!("Level: {}", aps_levels[1]);
        }
    if employee.1 == "Associate" {
            
            println!("Level: {}", aps_levels[2]); 
            
            
            println!("This position falls under the entry-level slice: {:?}", entry_level_slice);
        }
    if employee.1 == "Senior Associate 1-2" {
            println!("Level: {}", aps_levels[3]);
        }
    if employee.1 == "Senior Associate 3-4" {
            println!("Level: {}", aps_levels[4]);
        }
    if employee.1 == "Partner" {
            println!("Level: {}", aps_levels[5]);
        }
    }

    if employee.0 == "Office Admin" {
     if employee.1 == "Intern" {
            println!("Level: {}", aps_levels[0]);
        }
    if employee.1 == "Administrator" {
        println!("Level: {}", aps_levels[1]);
        }
    if employee.1 == "Senior Administrator" {
            println!("Level: {}", aps_levels[2]);
        }
     if employee.1 == "Office Manager" {
            println!("Level: {}", aps_levels[3]);
        }
    if employee.1 == "Director" {
            println!("Level: {}", aps_levels[4]);
        }
     if employee.1 == "CEO" {
        println!("Level: {}", aps_levels[5]);
        }
    }
}

 