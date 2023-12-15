use std::fs;

pub mod config;
pub mod argparser;

pub use argparser::*;
pub use config::*;

pub struct Hat;

impl Hat {
  pub fn read_to_string(config: &Config) -> String {
    let contents = fs::read_to_string(&config.path)
      .expect("File does not exists");

    let lines = contents
      .split("\n")
      .collect::<Vec<&str>>()[..config.total_lines as usize]
      .join("\n");

    lines
  }
}
