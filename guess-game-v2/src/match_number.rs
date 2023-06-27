use rand::Rng;
use std::cmp::Ordering;
mod user_input;
// mod check_value;

pub fn match_number() {
  let lucky_number = rand::thread_rng().gen_range(6..100);
  println!("{}", lucky_number);
  
  loop {
    let input_value = user_input::user_input();

    let check_input: u32 = match input_value.parse() {
    Ok(num) => num,
    Err(_)=> {
      println!("Please input a number as guess");
      continue;
    }
    };

    let mut _new_lucky = 0;
    let mut _new_input = 0;
  
    if lucky_number > 90 {
      _new_lucky = lucky_number
    }else {
        _new_lucky = lucky_number + 10
    }
  
    if check_input < 13 {
      _new_input = 10
    } else {
      _new_input = lucky_number - 12
    }   

    match check_input.cmp(&lucky_number) {
      Ordering::Greater => {
        if check_input >= 100 {
          println!("It's too big, guess a number under 100")
        }else{
          println!("It's too big, guess a number between {} to {}", _new_input, _new_lucky)
        }
      },

      Ordering::Less => println!("It's too small, guess a number between {} to {}", _new_input, _new_lucky),

      Ordering::Equal => {
        println!("Equal");
        break;
      },
    }
}
}