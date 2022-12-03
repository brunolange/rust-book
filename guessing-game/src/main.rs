use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guessed_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        print!("You guessed: {guess}");

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!(">"),
            Ordering::Greater => println!("<"),
            Ordering::Equal => {
                println!("Got it");
                break;
            }
        };
    }
}
 