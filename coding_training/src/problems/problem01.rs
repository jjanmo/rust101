use std::io::stdin;

pub fn solve() {
  let mut name = String::new();
  stdin().read_line(&mut name).unwrap();

  let name = name.trim();
  let question = format!("What is your name? {}", name);
  let welcome = format!("Hello, {}, nice to meet you!", name);

  println!("{}", question);
  println!("{}", welcome);
}