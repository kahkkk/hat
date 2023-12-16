use std::collections::HashMap;

pub struct Config {
  pub path: String,
  pub total_lines: u32
}

impl Config {
  pub fn build(args: HashMap<String, String>) -> Config {
    let path = args
      .get("file")
      .expect("File path must be required.");

    let total_lines = args
      .get("lines")
      .and_then(|n| Some(n.parse::<u32>().unwrap()))
      .unwrap_or(10);

    Config { path: String::from(path), total_lines }
  }
}
