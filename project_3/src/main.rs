use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("The_EFCC_Merge.csv").expect("Write failed");

    file.write_all("Commissioner,Ministry,Zone\n".as_bytes()).expect("Write failed");

    file.write_all("Aigbogun Alamba Daudu,Internal Affairs,South West\n".as_bytes()).expect("Write failed");
    file.write_all("Murtala Afeez Bendu,Justice,North East\n".as_bytes()).expect("Write failed");
    file.write_all("Okorocha Calistus Ogbona,Defense,South South\n".as_bytes()).expect("Write failed");
    file.write_all("Adewale Jimoh Akanbi,Power & Steel,South West\n".as_bytes()).expect("Write failed");
    file.write_all("Osazuwa Faith Etieye,Petroleum,South East\n".as_bytes()).expect("Write failed");

    println!("\nData written to file.");
}
