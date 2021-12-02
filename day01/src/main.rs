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

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<u32> {
  let before = std::time::Instant::now();
  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let content = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .map(|l| l.parse::<u32>().expect("Could not parse number"))
    .collect();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  content
}

fn part1(data: &Vec<u32>) -> () {
  let before = std::time::Instant::now();
  let mut previous_depth = &data[0];
  let mut result = 0;
  for depth in &data[1..] {
    if depth > previous_depth {
      result += 1;
    }
    previous_depth = &depth;
  }
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    result,
    before.elapsed()
  );
}

fn part2(data: &Vec<u32>) -> () {
  let before = std::time::Instant::now();
  let mut result = 0;

  for i in 3..data.len() {
    if data[i] > data[i - 3] {
      result += 1;
    }
  }
  println!(
    "Result part2: {:>10} | elapsed time: {:.2?}",
    result,
    before.elapsed()
  );
}
