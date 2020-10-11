use crate::puzzle_utils;

pub fn count(filename: &str) -> i32 {
  let mut total = 0;
  if let Ok(lines) = puzzle_utils::read_lines(filename) {
    for line in lines {
      if let Ok(mass_note) = line {
        if let Ok(mass) = mass_note.parse::<f32>() {
          total += get_module_fuel(mass);
        }
      }
    }
  }
  total
}

fn get_module_fuel(mass: f32) -> i32 {
  let current = ((mass / 3.0).floor() - 2.0) as i32;
  if current > 0 {
    current + get_module_fuel(current as f32)
  } else {
    0
  }
}
