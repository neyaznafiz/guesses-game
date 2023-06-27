use super::user_input;

pub fn check_value() {
  let input_value = user_input::user_input();
  let check_input: u32 = match input_value.parse() {
    Ok(num) => num,
    Err(_)=> {
     return println!("Please input a number as guess");
    }
  };
}