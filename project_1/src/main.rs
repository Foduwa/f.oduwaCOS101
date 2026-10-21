use std::io::Write;

fn main() {
   
    let headers = "Drink Name,Category\n";
    
    
    let mut file = std::fs::File::create("nigerian_breweries.csv").expect("create failed");

    
    file.write_all(headers.as_bytes()).expect("write failed");

    file.write_all("33 Export,Lager\n".as_bytes()).expect("write failed");
    file.write_all("Desperados,Lager\n".as_bytes()).expect("write failed");
    file.write_all("Goldberg,Lager\n".as_bytes()).expect("write failed");
    file.write_all("Gulder,Lager\n".as_bytes()).expect("write failed");
    file.write_all("Heineken,Lager\n".as_bytes()).expect("write failed");
    file.write_all("Star,Lager\n".as_bytes()).expect("write failed");

    // stout 
    file.write_all("Legend,Stout\n".as_bytes()).expect("write failed");
    file.write_all("Turbo King,Stout\n".as_bytes()).expect("write failed");
    file.write_all("Williams,Stout\n".as_bytes()).expect("write failed");

    // non alcohols
    file.write_all("Maltina,Non-Alcoholic\n".as_bytes()).expect("write failed");
    file.write_all("Amstel Malta,Non-Alcoholic\n".as_bytes()).expect("write failed");
    file.write_all("Malta Gold,Non-Alcoholic\n".as_bytes()).expect("write failed");
    file.write_all("Fayrouz,Non-Alcoholic\n".as_bytes()).expect("write failed");

   
    println!("\nData written to file." );
}
