use rand::Rng;
mod user_input;
mod check_value;
mod user_hint;

pub fn match_number() {
  let lucky_number = rand::thread_rng().gen_range(6..100);
  let mut input_number = 0;
  let mut big_number = 0;
  let mut small_number = 0;
  println!("Please guess your lucky number between 1 to 100 ! {}", lucky_number);
  
  loop {
    
    // input check
    let check_input = check_value::check_value();
    let user_input: u32;
    match check_input {
      Ok(val) => { 
        input_number += 1;
        user_input = val;
      }
      Err(error) => {
        input_number += 1;
        println!("{error}");
        continue;
      }
    }
    
    
    let _input_difference = user_hint::user_hint(big_number, small_number);


    // input value compare with lucky number
    if user_input > 100 {
      println!("It's too big, guess a number under 100")
    } 
    else if lucky_number < user_input {
      big_number = user_input;
      small_number = lucky_number;
      println!("It's too big");
    } 
    else if lucky_number > user_input{
      println!("It's too small");
      big_number = lucky_number;
      small_number = user_input;
    }
    else {
      println!("Congratulation you win - You have tried {input_number} times.");
      break;
    }
  }
}

