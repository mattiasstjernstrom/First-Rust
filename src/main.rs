use std::io;

fn main() {
    println!("Type something:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("You typed: {}", input);
}
