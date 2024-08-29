use std::io;

fn main() {
    println!("Guess the secret number!");

    println!("Please input your guess.");

    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Failed to take input");

    let num:i32 = num.trim().parse().expect("enter a valid number");

    println!("You guessed: {}", num);
}