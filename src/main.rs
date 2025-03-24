use rand::Rng;
use std::arch::x86_64;
use std::cmp::Ordering;
use std::num::NonZeroI32;
use std::{i64, io};

fn main() {
    // guessing_number()
    variable()
}

fn guessing_number() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input a Number to guess!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
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

fn variable() {
    let x = 5;
    println!("Value is: {x}");
    let x: i64 = 7;
    println!("value is before scope: {x}");
    {
        let x: i32 = 43;

        println!("Value is after scope: {x}");
    }
    println!("value without scope: {x}");

    let x: bool = true;

    println!("value of boolean: {x}");

    let x: f32 = 1.5;
    println!("Value of float: {x}");
}
