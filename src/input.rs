// To get input from users

use std::io;

pub fn run(){
    let mut line = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut line).expect("Error");
    print!("Hello, {}",line);
}