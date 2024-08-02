use std::io;
use rand::Rng;

fn main() {
    println!("Number Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);  // 左闭右开
    println!("Secret Number is: {}", secret_number);

    println!("Enter your estimated number:");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Cannot Read Line.");

    println!("The number you guessed is: {}", guess);
}
