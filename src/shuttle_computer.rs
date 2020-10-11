use crate::puzzle_utils;
use std::str::FromStr;

fn read_values<T: FromStr>(s: String) -> Result<Vec<T>, T::Err> {
  s.trim().split(',').map(|word| word.parse()).collect()
}

enum Opcode {
  Addition,
  Multiply,
  Halt,
}

struct Instruction(Opcode, i32, i32, usize);

impl Instruction {
  pub fn new(opcode: &i32, first: i32, second: i32, address: usize) -> Self {
    Self(
      match opcode {
        1 => Opcode::Addition,
        2 => Opcode::Multiply,
        99 => Opcode::Halt,
        _ => panic!("Wrong opcode: {}", opcode),
      },
      first,
      second,
      address,
    )
  }
}

pub fn shuttle_magic(filename: &str, pair_output: i32) {
  if let Ok(data_input) = puzzle_utils::read_to_str(filename) {
    // println!("Input: {}", data_input);
    let values = read_values::<i32>(data_input).unwrap();
    let step = 4;
    for noun in 0..99 {
      for verb in 0..99 {
        let mut inputs = values.clone();
        inputs[1] = noun;
        inputs[2] = verb;
        for (i, val) in inputs.clone().iter().step_by(step).enumerate() {
          let index = i * step;
          let instruction = Instruction::new(
            val,
            inputs[inputs[index + 1] as usize],
            inputs[inputs[index + 2] as usize],
            inputs[index + 3] as usize,
          );
          match instruction.0 {
            Opcode::Addition => {
              inputs[instruction.3] = instruction.1 + instruction.2;
            }
            Opcode::Multiply => {
              inputs[instruction.3] = instruction.1 * instruction.2;
            }
            Opcode::Halt => {
              break;
            } // _ => println!("Unhandled value: {}", val),
          }
          if inputs[0] == pair_output {
            println!(
              "Pair output: {}, where noun {} and verb {}",
              pair_output, noun, verb
            );
            println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
          }
        }
      }
    }
  }
}
