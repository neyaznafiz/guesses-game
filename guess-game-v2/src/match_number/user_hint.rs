pub fn user_hint(big: u32, small: u32) -> u32 {
  let difference = big - small;
  let from_difference = difference / 4;
  // println!("{}, {}, {big}, {small}", difference, from_difference);
  return from_difference;
}