use rand::Rng;
use std::io::Write;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("+------------------------+");
    println!("| TIME TO GUESS A NUMBER |");
    println!("+------------------------+");
    println!("");

    let secret_number = rand::thread_rng().gen_range(1 ..= 100);

    loop {
        print!("Enter a number: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("What happened to stdin?");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's no number!");
                continue;
            }
        };

        println!("You guessed the number {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Aim higher"),
            Ordering::Greater => println!("Whoa there"),
            Ordering::Equal => {
                println!("Nailed it!");
                break;
            }
        }
    }
}
