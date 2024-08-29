use rand::Rng; // trait
use std::cmp::Ordering;
use std::io; // prelude

fn main() {
    println!("Guess a number:");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
