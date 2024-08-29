use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the secret number!");

    

    let random_number = rand::thread_rng().gen_range(1..=100);
   
    println!("secret number is : {}", random_number);
    loop{
    println!("Please input your guess.");
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Failed to take input");

    let num:i32 = num.trim().parse().expect("enter a valid number");

    println!("You guessed: {}", num);

    match num.cmp(&random_number){
        Ordering::Less => println!("wrong:Less than the secret number!"),
        Ordering::Greater => println!("wrong:greater than the secret number!"),
        Ordering::Equal => {
            println!("Correct!");
            break;
    }
}}
}