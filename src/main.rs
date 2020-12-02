#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate num;

mod day1;
mod day2;
mod utils;

fn run() -> Result<(), utils::Error> {
  day2::problem1()?;
  day2::problem2()?;

  if false {
  }
  Ok(())
}

fn main() {
  match run() {
    Err(err) => println!("Error occurred: {}", err),
    _ => {}
  }
}
