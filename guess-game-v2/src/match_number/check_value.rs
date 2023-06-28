use super::user_input;

pub fn check_value() -> Result<u32, String> {
  let input_value = user_input::user_input();
  match input_value.parse() {
    Ok(num) => Ok(num),
    Err(_) => {Err("Please input a number as guess".to_string())}
  }
}