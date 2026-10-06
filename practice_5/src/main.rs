fn mutate_num_to_zero(param_num: i32) {
    let param_num = param_num * 0;
    println!("param_num is {}", param_num);
}

fn main() {
    let num = 5;
    mutate_num_to_zero(num);
    println!("num is {}", num);
}

