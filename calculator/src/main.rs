use std::io;
fn main() {
    println!("input two numbers:");
    let mut num1 = String::new();
    let mut num2 = String::new();

    io::stdin().read_line(&mut num1).expect("failed");
    io::stdin().read_line(&mut num2).expect("failed");
    // In rust, we cannot apply mathematical operations without parsing the string.
    let num1:i32 = num1.trim().parse().expect("enter a valid number");
    let num2:i32 = num2.trim().parse().expect("enter a valid number");
    let sum = num1 + num2;
    println!("The sum is: {}", sum);
}
