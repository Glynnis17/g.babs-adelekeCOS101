use std::io::Write;

fn main() {
    let smis_matrix = vec![
    ["Student Name", "Matric. Number", "Department", "Level"],
    ["Oluchi Mordi", "ACC10211111", "Accounting", "300"],
    ["Adams Aliyu", "ECO10110101", "Economics", "100"],
    ["Shania Bolade", "CSC10328828", "Computer", "200"],
    ["Adekunle Gold", "EEE11020202", "Electrical", "200"],
    ["Bianca Edemoh", "MEE10202001", "Mechanical", "100"],
];
    let mut file = std::fs::File::create("PAU-SMIS.csv").expect("create failed");

    for row in smis_matrix {
        let line = format!("{},{},{},{}\n", row[0], row[1], row[2], row[3]);
     file.write_all(line.as_bytes()).expect("write failed");
    }
    

    println!("\nData written to file");

}
