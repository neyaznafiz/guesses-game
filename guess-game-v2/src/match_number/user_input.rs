use std::io;

pub fn user_input() -> String {
  // println!("Please input your guess");
    let mut input: String =  String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    return input.trim().to_string();
}