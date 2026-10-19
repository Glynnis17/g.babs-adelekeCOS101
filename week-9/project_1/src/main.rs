use std::io::Write;


fn main() {
    let headers = "Lager, Stout, Non-Alcoholic\n";
    let data_1 = "33 Export, Legend, Maltina\n";
    let data_2 = "Desperados, Turbo King, Amstel Malta\n";
    let data_3 = "Goldberg, Williams, Malta King\n";
    let data_4 = "Gulder,  , Fayrouz\n";
    let data_5 = "Heineken, , ,\n";
    let data_6 = "Star, , ,\n";


    let mut file = std::fs::File::create("Nigerian_Breweries.csv").expect("create failed");
    file.write_all(headers.as_bytes()).expect("write failed");
    file.write_all(data_1.as_bytes()).expect("write failed");
    file.write_all(data_2.as_bytes()).expect("write failed");
    file.write_all(data_3.as_bytes()).expect("write failed");
    file.write_all(data_4.as_bytes()).expect("write failed");
    file.write_all(data_5.as_bytes()).expect("write failed");
    file.write_all(data_6.as_bytes()).expect("write failed");
    println!("\nData wriiten to file");
}
