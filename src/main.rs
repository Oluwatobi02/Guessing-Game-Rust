use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Less than the number!"),
        Ordering::Greater => println!("Greater than the number!"),
        Ordering::Equal => println!("Correct!"),

    }

    println!("You guessed: {}", guess);
    println!("The secret number is: {secret_number}");
}
