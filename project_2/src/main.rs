fn main() {
    
    let candidates: Vec<(&str, i32)> = vec![
        ("Chidi", 3),
        ("Amina", 8),
        ("Tunde", 5),
        ("Blessing", 12),
        ("Fatima", 6),
    ];

    
    let mut highest_years: i32 = 0;
    let mut highest_name: &str = "";

    for candidate in candidates {
        if candidate.1 > highest_years {
            highest_years = candidate.1;
            highest_name = candidate.0;
        }
    }

    println!("The developer with the highest years of programming experience is {} with {} years of experience.", highest_name, highest_years);
}
