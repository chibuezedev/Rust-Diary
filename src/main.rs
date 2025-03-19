use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    guessing_number()
}

fn guessing_number() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret Number is: {secret_number}");

    loop {
        println!("Input a Number to guess!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Greater, Try again!"),
            Ordering::Less => println!("Guess is Less, Try again!"),
            Ordering::Equal => {
                println!("Got it, You are Credible!!");
                break;
            }
        }
    }
}
