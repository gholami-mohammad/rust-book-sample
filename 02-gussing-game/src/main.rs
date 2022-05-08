use colored::{self, Colorize};
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the a number between 1 and 10!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..11);
    println!("secret number is: {}", secret_number);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You win!!!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Less => println!("{}", "Too small".red()),
        }
    }
}
