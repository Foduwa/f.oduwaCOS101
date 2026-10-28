struct Employee {
    ceo: String,
    company: String,
    age: u32,
}

fn main() {
    let emp1 = Employee {
        company: String::from("Microsoft Corporation"),
        ceo: String::from("Satya Nadella"),
        age: 56,
    };
    let emp2 = Employee {
        company: String::from("Google Inc."),
        ceo: String::from("Sundai Pichai"),
        age: 51,
    };
    
    // Pass by reference (&) so ownership isn't lost
    display(&emp1);
    display(&emp2);
}

// Update to accept a reference (&Employee)
fn display(emp: &Employee) {
    println!("Name is {} company is {} age is {}", emp.ceo, emp.company, emp.age);
}