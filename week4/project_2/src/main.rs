use std::io;

fn main() {
    let mut input = String::new();

    // Read experience (true/false)
    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut input).unwrap();
    let experienced = input.trim().to_lowercase() == "yes";
    input.clear();

    // Read age
    println!("Enter employee's age:");
    io::stdin().read_line(&mut input).unwrap();
    let age: i32 = input.trim().parse().unwrap();

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
            // If age is between 28–29, not explicitly listed, we can handle it here
            incentive = 1_300_000; // or adjust if your teacher specifies differently
        }
    } else {
        incentive = 100_000;
    }

    println!("Annual incentive: ₦{}", incentive);
}