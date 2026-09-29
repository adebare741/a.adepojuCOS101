use std::io;

fn main() {
    // Read a
    println!("Enter value of a:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: f64 = input1.trim().parse().expect("Failed to parse a");

    // Read b
    println!("Enter value of b:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: f64 = input2.trim().parse().expect("Failed to parse b");

    // Read c
    println!("Enter value of c:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c: f64 = input3.trim().parse().expect("Failed to parse c");

    // Calculate discriminant
    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct roots: {} and {}", root1, root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    } else {
        println!("No real roots.");
    }
} 