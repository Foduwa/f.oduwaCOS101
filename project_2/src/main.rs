use std::io::Write;

fn main() {

    let headers = "Student Name,Matric.Number,Department,Level\n";

    let mut file = std::fs::File::create("PAU-SMIS.CSC").expect("create failed");

    file.write_all(headers.as_bytes()).expect("write failed");

   
 //Matric  number



    file.write_all("ACC10211111,Matric.Number\n".as_bytes()).expect("write failed");
    file.write_all("ECO10110101,Matric.Numbe\n".as_bytes()).expect("write failed");
    file.write_all("CSC10328828,Matric.Number\n".as_bytes()).expect("write failed");
     file.write_all("EEE11020202,Matric Number\n".as_bytes()).expect("write failed");
     file.write_all("MEE10202001,Matric number\n".as_bytes()).expect("write failed");

 
//student name 

    file.write_all("Oluchi Mordi,Student Name\n".as_bytes()).expect("write failed");
    file.write_all("Adams Aliyu,Student Name\n".as_bytes()).expect("write failed");
     file.write_all("Shania Bolade,Student Name\n".as_bytes()).expect("write failed");
     file.write_all("Adekunle Gold,Student Name\n".as_bytes()).expect("write failed");
     file.write_all("Bianca Edemoh,Student Name\n".as_bytes()).expect("write failed");

// DEPARTMENT
 
    file.write_all("Accounting,Department\n".as_bytes()).expect("write failed");
    file.write_all("Economics,Department\n".as_bytes()).expect("write failed");
     file.write_all("Computer,Department\n".as_bytes()).expect("write failed");
     file.write_all("Electrical,Department\n".as_bytes()).expect("write failed");
     file.write_all("Mechanical,Department\n".as_bytes()).expect("write failed");

     //level


    file.write_all("300,level\n".as_bytes()).expect("write failed");
    file.write_all("100,level\n".as_bytes()).expect("write failed");
     file.write_all("200,level\n".as_bytes()).expect("write failed");
     file.write_all("200,level\n".as_bytes()).expect("write failed");
     file.write_all("100,level\n".as_bytes()).expect("write failed");

     println!("\nData written to file.");
}

    