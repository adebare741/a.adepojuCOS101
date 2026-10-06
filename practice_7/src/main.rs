fn main() {
    let arr1: [i32; 4] = [10, 20, 30, 40];
    let arr2 = [10.4, 20.7, 30.9];
    let arr3: [i32; 8] = [-1; 8];

    println!("{:?}, length = {}", arr1, arr1.len());
    println!("{:?}, length = {}", arr2, arr2.len());
    println!("{:?}, length = {}", arr3, arr3.len());
}
