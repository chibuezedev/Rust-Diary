use rand::Rng;
use std::cmp::Ordering;
use std::{i64, io};

fn main() {
    array_check();
    // guessing_number()
    // variable()
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
    // scoping
    let x = 5;
    println!("Value is: {x}");
    let x: i64 = 7;
    println!("value is before scope: {x}");
    {
        let x: i32 = 43;

        println!("Value is after scope: {x}");
    }
    println!("value without scope: {x}");

    // boolean
    let x: bool = true;

    println!("value of boolean: {x}");

    // character
    let x: f32 = 1.5;
    println!("Value of float: {x}");
    let character: &str = "Hello world!";
    println!("Character is: {character}");

    // tuple
    let tup = (500, 4.4, 1);
    let dot: f64 = tup.1;
    println!("using dot notation: {dot}");
    let (x, y, z) = tup;
    println!("Value in Y IS: {y}")
}

fn array_check() {
    let array = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();
    let msg = "Failed to read line!";
    io::stdin().read_line(&mut index).expect(msg);

    let index: usize = index.trim().parse().expect("Index passed is not a number");

    let element = array[index];

    println!("The value of the element at the index: {index} is {element}")
}
