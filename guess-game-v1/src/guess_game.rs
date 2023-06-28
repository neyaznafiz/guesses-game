use rand::Rng;
use std::io;
use std::cmp::Ordering;

pub fn guess() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

  println!("Please input your guess");

  // loop for check number
  loop {
    let mut guesses = String::new();
    io::stdin()
        .read_line(&mut guesses)
        .expect("Failed to read line");

    let guesses: u32 = match guesses.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Pleae input a number as guess!");
				continue;
			},
		};

    // println!("You guessed: {guesses}");

    // check number value
    match guesses.cmp(&secret_number) {
        Ordering::Less => println!("Your number {guesses} is too small than secret number {secret_number}"),
        Ordering::Greater => println!("Your number {guesses} is too big than secret number {secret_number}"),
        Ordering::Equal => {
					println!("YEY! You gussed the right number.");
					break;
				}
    }
  }
}
