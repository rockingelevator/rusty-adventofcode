#[allow(dead_code)]
use std::env;

// mod fuel_counter;
mod puzzle_utils;
mod shuttle_computer;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename;
  let mut pair_output: i32 = 0;
  match args.len() {
    2 => {
      filename = &args[1];
    }
    3 => {
      filename = &args[1];
      pair_output = args[2].parse().unwrap();
    }
    _ => panic!("Missing parameters"),
  }

  shuttle_computer::shuttle_magic(filename, pair_output);

  // Day 1: https://adventofcode.com/2019/day/1
  // let total_fuel = fuel_counter::count(out_filename);
  // println!("Total fuel: {}", total_fuel);
}
