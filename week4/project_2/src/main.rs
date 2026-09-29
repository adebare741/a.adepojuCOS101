// Rust program to count numbers
use std::io;

fn main() {
    // Read experience
    println!("Is the employee experienced? (yes/no)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experienced = input1.trim().to_lowercase() == "yes";

    // Read age
    println!("Enter employee's age:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age: i32 = input2.trim().parse().expect("Failed to parse age");

    // Decision making
    let incentive: i32;

    if experienced {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age <= 39 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            // Handle ages not explicitly listed (like 28–29)
            incentive = 1_300_000;
        }
    } else {
        incentive = 100_000;
    }

    println!("Annual incentive is ₦{}", incentive);