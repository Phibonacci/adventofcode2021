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

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<Screen> {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let data = buf
    .lines()
    .map(|l| {
      l.unwrap()
        .split('|')
        .map(|side| {
          side
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<InOut>()
        })
        .collect::<Screen>()
    })
    .collect::<Vec<Screen>>();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  data
}

type InOut = Vec<String>;
type Screen = Vec<InOut>;

fn part1(data: &Vec<Screen>) -> () {
  let before = std::time::Instant::now();
  let mut result = 0;
  for screen in data {
    for output in &screen[1] {
      match output.len() {
        2 | 4 | 3 | 7 => result += 1, // 1, 4, 7, 8
        _ => (),
      }
    }
  }
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    result,
    before.elapsed()
  );
}

// the mean seems to be very close from the expected results so I just iterate from there
fn part2(data: &Vec<Screen>) -> () {
  let before = std::time::Instant::now();
  let mut result = 0;
  for screen in data {
    result += decode(screen);
  }
  println!(
    "Result part2: {:>10} | elapsed time: {:.2?}",
    result,
    before.elapsed()
  );
}

fn decode(screen: &Screen) -> u64 {
  let mut digits = vec![None; 10];
  let mut harder_to_guess = Vec::new();
  for input in &screen[0] {
    match input.len() {
      2 => digits[1] = Some(input), // 1
      4 => digits[4] = Some(input), // 4
      3 => digits[7] = Some(input), // 7
      7 => digits[8] = Some(input), // 8
      _ => harder_to_guess.push(input),
    }
  }
  for input in &harder_to_guess {
    if input.len() == 5 {
      if contains_characters(input, digits[1].unwrap()) {
        digits[3] = Some(input);
      } else if missing_characters(input, digits[4].unwrap()) == 1 {
        digits[5] = Some(input);
      } else {
        digits[2] = Some(input);
      }
    } else {
      if contains_characters(input, digits[4].unwrap()) {
        digits[9] = Some(input);
      } else if contains_characters(input, digits[7].unwrap()) {
        digits[0] = Some(input);
      } else {
        digits[6] = Some(input);
      }
    }
  }
  let mut result = 0;
  for output in &screen[1] {
    for i in 0..digits.len() {
      if is_digit(output, digits[i].unwrap()) {
        result *= 10;
        result += i as u64;
        break;
      }
    }
  }
  result
}

fn is_digit(a: &str, b: &str) -> bool {
  if a.len() != b.len() {
    return false;
  }
  return contains_characters(a, b);
}

fn contains_characters(a: &str, b: &str) -> bool {
  for c in b.chars() {
    if !a.contains(c) {
      return false;
    }
  }
  return true;
}

fn missing_characters(a: &str, b: &str) -> u64 {
  let mut missing = 0;
  for c in b.chars() {
    if !a.contains(c) {
      missing += 1;
    }
  }
  missing
}
