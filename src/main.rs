#![allow(dead_code)]

extern crate lazy_static;
extern crate regex;
extern crate num;

mod day1;
mod utils;

fn run() -> Result<(), utils::Error> {
  day1::problem1();
  day1::problem2();

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
