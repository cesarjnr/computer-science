use std::io;

fn main() {
  println!("Please input your name:");

  let mut name = String::new();

  io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");

  println!("Your name is: {name}");
}