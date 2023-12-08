use std::fs;
use std::env;

// cargo run -- <FILE> [lines] <TOTAL = 10>
fn main() {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];
  let total_lines: i32;

  if args.len() == 3 {
    total_lines = args[2].parse().unwrap();
  } else {
    total_lines = 10;
  }

  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
  let lines: Vec<&str> = contents.split("\n").collect();

  let mut idx: i32 = 0;
  let mut text: String = "".to_owned();

  while idx < total_lines {
    text.push_str("\n");
    text.push_str(&lines[idx as usize]);
    idx += 1;
  }
  
  print!("{}", text);
}
