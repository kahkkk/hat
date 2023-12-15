use hat::{ Config, Parser, Hat };

fn main() {
  let config = Config::build(Parser::read_args());
  let lines = Hat::read_to_string(&config);

  print!("{}", lines)
}
