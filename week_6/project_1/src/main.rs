use std::io;

fn main() {
    // Step 1: Display Menu (String literals + format!)
    println!("Welcome to Pan-Atlantic Restaurant!");
    println!("Menu:");
    println!("P: Poundo Yam / Edinkaiko Soup - ₦3200");
    println!("F: Fried Rice & Chicken - ₦3000");
    println!("A: Amala & Ewedu Soup - ₦2500");
    println!("E: Eba & Egusi Soup - ₦2000");
    println!("W: White Rice & Stew - ₦2500");

    // Step 2: Read input (String object)
    let mut food = String::new();
    println!("Enter food type (P/F/A/E/W): ");
    io::stdin().read_line(&mut food).expect("Failed to read input");
    let food = food.trim(); // trim() practice

    let mut qty_input = String::new();
    println!("Enter quantity: ");
    io::stdin().read_line(&mut qty_input).expect("Failed to read input");
    let qty: i32 = qty_input.trim().parse().expect("Please enter a number");

    // Step 3: Decision with relational operators
    let price: i32 = match food {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid choice!");
            return;
        }
    };

    // Step 4: Arithmetic operators
    let total = price * qty;

    // Step 5: Discount rule (relational + arithmetic)
    let final_total = if total > 10_000 {
        total - (total * 5 / 100) // 5% discount
    } else {
        total
    };

    // Step 6: Output with format!
    println!("{}", format!("Total charge: ₦{}", final_total));
}

