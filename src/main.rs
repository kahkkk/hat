use std::fs;
use std::env;
use std::collections::HashMap;

fn read_args() -> HashMap<String, String> {
  let args: Vec<String> = env::args().collect();

  let mut idx: usize = 1;
  let mut params = HashMap::new();
  
  while idx < args.len() {
    let curr = &args[idx];

    if curr.starts_with("--") {
      let has_next = idx + 1 < args.len();
      let is_boolean = !has_next || args[idx + 1].starts_with("--");

      if has_next && !is_boolean {
        params.insert(curr.to_owned(), args[idx + 1].to_owned());
        idx += 2;
      } else if is_boolean {
        params.insert(curr.to_owned(), String::from("true").to_owned());
        idx += 1;
      }
    } else {
      idx += 1;
    }
  }

  params
}

fn main() {
  let params = read_args();
  
  let file_path = params
    .get("--file")
    .expect("File path is required");

  let total_lines: i32 = params
    .get("--lines")
    .and_then(|n| Some(n.parse::<i32>().unwrap()))
    .unwrap_or(10);

  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

  let lines = &contents
    .split("\n")
    .collect::<Vec<&str>>()[..total_lines as usize]
    .join("\n");

  print!("{}", lines)
}
