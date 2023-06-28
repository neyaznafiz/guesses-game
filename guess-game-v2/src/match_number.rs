use rand::Rng;
mod user_input;
mod check_value;
use std::{cmp::Ordering};

use crate::match_number::user_input::user_input;


pub fn match_number() {
  let lucky_number = rand::thread_rng().gen_range(6..100);
  let mut input_number = 0;
  println!("Please guess your lucky number between 1 to 100 ! {}", lucky_number);
  
  loop {
    let check_input = check_value::check_value();

    let user_input: u32;
    match check_input {
      Ok(val) => { 
        input_number = input_number + 1;
        user_input = val; 
      }
      Err(error) => {
        input_number = input_number + 1;
        println!("{error}");
        continue;
      }
    }

    
    if lucky_number < user_input {
      println!("It's too big");
    } else if lucky_number > user_input{
      println!("It's too small");
    } else {
      println!("YEY! Congratulation, you tried {input_number} times.")
    }





    // if   user_input == _input_value {
    //   user_input =  check_input.ok().unwrap();
    // } else {
    //     // check_input.err();
    //     println!("{:?}",  check_input.err());
    // }

    // let mut _new_lucky = 0;
    // let mut _new_input = 0;
  
    // if lucky_number > 90 {
    //   _new_lucky = lucky_number
    // }else {
    //     _new_lucky = lucky_number + 10
    // }
  
    // if user_input < 13 {
    //   _new_input = 10
    // } else {
    //   _new_input = lucky_number - 12
    // }   

    // match user_input.cmp(&lucky_number) {
    //   Ordering::Greater => {
    //     if user_input >= 100 {
    //       println!("It's too big, guess a number under 100")
    //     } else{
    //       println!("It's too big, guess a number between {} to {}", _new_input, _new_lucky)
    //     }
    //   },

    //   Ordering::Less => println!("It's too small, guess a number between {} to {}", _new_input, _new_lucky),

    //   Ordering::Equal => {
    //     println!("Equal");
    //     break;
    //   },
    // }
  }
}