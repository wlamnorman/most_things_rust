use std::io;
use rand::prelude::*;

fn main() {
    let number = rand::thread_rng().gen_range(0..101);
    println!("True number is {}", number);
    println!("Enter a number in the range 0-100: ");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read the guess.");
        let guess: i32 = guess.trim().parse().unwrap(); //.except("Failed to pase the guess.");
        if guess > number {
            println!("Too high! Guess again!");
        } else if guess < number {
            println!("Too low! Guess again!");
        } else {
            println!("You got it! Congratulations!");
            break;
        }
    }
}