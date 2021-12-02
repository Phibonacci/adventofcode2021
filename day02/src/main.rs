fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let data = parse_file(filename);
  part1(&data);
  part2(&data);
  println!("Total elapsed time: {:.2?}", before.elapsed());
}

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<(Order, u32)> {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let content = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .map(|l| parse_line(&l))
    .collect();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  content
}

enum Order {
  Forward,
  Down,
  Up,
}

fn parse_line(line: &String) -> (Order, u32) {
  let tokens = line.split_whitespace().collect::<Vec<&str>>();
  let direction = match tokens[0] {
    "forward" => Order::Forward,
    "down" => Order::Down,
    "up" => Order::Up,
    _ => panic!(),
  };
  let value = tokens[1].parse::<u32>().unwrap();
  (direction, value)
}

fn part1(data: &Vec<(Order, u32)>) -> () {
  let before = std::time::Instant::now();
  let mut horizontal = 0;
  let mut depth = 0;
  for action in data {
    match action.0 {
      Order::Forward => horizontal += action.1,
      Order::Down => depth += action.1,
      Order::Up => depth -= action.1,
    }
  }
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    horizontal * depth,
    before.elapsed()
  );
}

fn part2(data: &Vec<(Order, u32)>) -> () {
  let before = std::time::Instant::now();
  let mut horizontal = 0;
  let mut aim = 0;
  let mut depth = 0;
  for action in data {
    match action.0 {
      Order::Forward => {
        horizontal += action.1;
        depth += aim * action.1;
      }
      Order::Down => aim += action.1,
      Order::Up => aim -= action.1,
    }
  }
  println!(
    "Result part2: {:>10} | elapsed time: {:.2?}",
    horizontal * depth,
    before.elapsed()
  );
}
