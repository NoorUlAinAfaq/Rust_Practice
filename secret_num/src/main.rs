use std::io;
use rand::Rng;


fn main() {
    println!("Guess the secret number!");

    println!("Please input your guess.");

    let mut num = String::new();

   let random_number = rand::thread_rng().gen_range(1..=100);
   
    println!("secret number is : {}", random_number);

    io::stdin().read_line(&mut num).expect("Failed to take input");

    let num:i32 = num.trim().parse().expect("enter a valid number");

    println!("You guessed: {}", num);
}