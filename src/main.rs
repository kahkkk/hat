use hat::{ Config, Hat };
use phosphophyllite::Parser;

fn main() {
  let mut parser = Parser::new();
  parser.add_arg("file");
  parser.add_arg("lines");

  let config = Config::build(parser.parse());
  let lines = Hat::read_to_string(&config);

  print!("{}", lines)
}
