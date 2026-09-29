fn main() {
    let n1 = String::from("Pan-");
    let n2 = String::from("Atlantic ");
    let n3 = String::from("University");

    let n4 = n1 + &n2 + &n3;
    println!("Concatenated: {}", n4);
}
