use std::env;

mod puzzle_utils;
// mod fuel_counter; // Day 1
mod shuttle_computer; // Day 2

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename;
  // Day 1: https://adventofcode.com/2019/day/1
  // $ cargo run data/day1_input.txt
  // let total_fuel = fuel_counter::count(out_filename);
  // println!("Total fuel: {}", total_fuel);

  // Day 2:
  // $ cargo run day2_input.txt 19690720
  let mut pair_output: i32 = 0;
  match args.len() {
    2 => {
      filename = &args[1];
    }
    3 => {
      filename = &args[1];
      pair_output = args[2].parse().unwrap();
    }
    _ => panic!("Check parameters"),
  }

  shuttle_computer::shuttle_magic(filename, pair_output);
}
