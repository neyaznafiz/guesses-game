use rand::Rng;
use std::io;
use std::cmp::Ordering;

pub fn guess() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guesses = String::new();
    io::stdin()
        .read_line(&mut guesses)
        .expect("Failed to read line");

    let guesses: u32 = guesses.trim().parse()
                       .expect("Please type a number");

    // println!("You guessed: {guesses}");

    match guesses.cmp(&secret_number) {
        Ordering::Less => println!("Your number {guesses} is too small than secret number {secret_number}"),
        Ordering::Greater => println!("Your number {guesses} is too big than secret number {secret_number}"),
        Ordering::Equal => println!("YEY! You gussed the right number."),
    }
}
