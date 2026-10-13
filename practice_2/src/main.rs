fn checker() {
    let ch = '5';
    if ch >= '0' && ch <= '9' {
        println!("{} is a digit", ch);
    } else {
        println!("{} is not a digit", ch);
    }
}

fn main() {
    println!("Welcome!");
    checker();
}
