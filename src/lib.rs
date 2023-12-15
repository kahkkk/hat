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

    let mut i: u32 = 0;
    let lines = contents
      .lines()
      .take_while(|_| { let tmp = i; i += 1; tmp < config.total_lines })
      .collect::<Vec<_>>()
      .join("\n");

    lines
  }
}
