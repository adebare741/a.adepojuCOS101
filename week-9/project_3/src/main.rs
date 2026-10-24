use std::fs::File;
use std::io::Write;
fn main() {
    let commissioners = vec![
        "Algorithm Algebra Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Ádewale Jimoh Akanbi",
        "Osezuma Faith Etieye",
    ];
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defence",
        "Power & Steel",
        "Petroleum",
    ];
    let zones = vec![ 
        "South West",
        "North West", 
        "South South",
        "South West",
        "South East",
    ];
    let mut file = File::create("efcc_records.txt")
        .expect("Failed to create file");
    for i in 0..commissioners.len() {
        file.write(commissioners[i].as_bytes())
            .expect("Write failed");
        file.write(", ".as_bytes())
            .expect("Write failed");
        file.write(ministries[i].as_bytes())
            .expect("Write failed");
        file.write(", ".as_bytes())
            .expect("Write failed"); 
        file.write(zones[i].as_bytes())       
            .expect("Write failed"); 
        file.write("\n".as_bytes())
            .expect("Write failed"); 
    }
    println!("EFCC records merged successfully!");
}            