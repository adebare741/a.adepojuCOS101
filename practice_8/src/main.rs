fn main() {
    let city_arr: [&str; 5] = ["Abuja", "Lagos", "Kano", "Ibadan", "Enugu"];

    for index in 0..5 {
        println!("{}", city_arr[index]);
    }
}
