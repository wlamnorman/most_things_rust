use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess a number between 1 and 100!");
    loop {
        println!("State your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You have to input a number between 1-100.");
                continue;
            }
        };
        // if Err (input value could not be parsed to u32) guess again

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed {} which is too small!", guess),
            Ordering::Equal => {
                println!("You guessed {} which is the correct number!", guess);
                break;
            }
            Ordering::Greater => println!("You guessed {} which is too large!", guess),
        }
    }
}
