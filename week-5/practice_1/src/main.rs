use std::io;
fn main() {
   let mut input1 = String::new();
   let mut input2 = String::new(); 
   let mut input3 = String::new();

   println!("Enter the value of a:");
   io::stdin().read_line(&mut input1).expect("invalid string");
   let a: f64 = input1.trim().parse().expect("invalid number");


   println!("Enter the value of b:");
   io::stdin().read_line(&mut input2).expect("invalid string");
   let b: f64 = input2.trim().parse().expect("invalid number");

