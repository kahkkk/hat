use std::env;
use std::collections::HashMap;

pub struct Parser {
  pub args: HashMap<String, String>
}

impl Parser {
  pub fn read_args() -> Parser {
    let mut params = env::args().skip(1);
    let mut args = HashMap::new();

    'outer: loop {
      match params.next() {
        Some(var) => {
          if var.starts_with("--") {
            args.insert(
              var.to_owned().to_string(),
              params.next().expect(&format!("The {} key must be have a value", &var).to_string())
            );
          }
        },
        None => break 'outer,
      }
    }

    Parser { args }
  }

  pub fn get(&self, key: &str) -> Option<&String> {
    self.args.get(key)
  }
}
