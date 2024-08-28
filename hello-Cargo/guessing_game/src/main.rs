use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Failed to take input");

    // io::stdin()
    //     .read_line(&mut num)
    //     .expect("Failed to read line");

    println!("You guessed: {}", num);
}