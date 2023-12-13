use std::fs;

pub mod argparser;

use argparser::{ Parser };

fn main() {
  let parser = Parser::read_args();
  
  let file_path = parser
    .get("--file")
    .expect("File path is required");

  let total_lines: u32 = parser 
    .get("--lines")
    .and_then(|n| Some(n.parse::<u32>().unwrap()))
    .unwrap_or(10);

  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

  let lines = &contents
    .split("\n")
    .collect::<Vec<&str>>()[..total_lines as usize]
    .join("\n");

  print!("{}", lines)
}
