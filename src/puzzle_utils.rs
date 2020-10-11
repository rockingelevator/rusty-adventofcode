use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
#[allow(dead_code)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

pub fn read_to_str<P>(filename: P) -> std::io::Result<String>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  let mut buf_reader = io::BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;
  Ok(contents)
}
