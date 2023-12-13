use std::env;
use std::collections::HashMap;

pub struct Parser {
  pub args: HashMap<String, String>
}

impl Parser {
  pub fn read_args() -> Parser {
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

    Parser { args: params }
  }

  pub fn get(&self, key: &str) -> Option<&String> {
    self.args.get(key)
  }
}
