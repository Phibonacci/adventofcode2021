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

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<String> {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let data = buf.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  data
}

fn part1(data: &Vec<String>) {
  let before = std::time::Instant::now();
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    data.iter().fold(0, |sum, l| sum + check_line(l)),
    before.elapsed()
  );
}

fn check_line(line: &str) -> u64 {
  let mut stack = Vec::new();
  for c in line.chars() {
    match c {
      '{' | '(' | '[' | '<' => stack.push(c),
      '}' | ')' | ']' | '>' => {
        if stack.is_empty() || !brackets_match(stack.pop().unwrap(), c) {
          return error_score(c);
        }
      }
      _ => panic!(),
    }
  }
  0
}

fn get_closing_backet(first: char) -> char {
  match first {
    '{' => '}',
    '(' => ')',
    '[' => ']',
    '<' => '>',
    _ => panic!(format!("unexpected '{}'", first)),
  }
}

fn brackets_match(first: char, last: char) -> bool {
  let expected_last = get_closing_backet(first);
  expected_last == last
}

fn error_score(c: char) -> u64 {
  match c {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    _ => panic!(),
  }
}

fn part2(data: &Vec<String>) {
  let before = std::time::Instant::now();
  let valid_lines = data
    .iter()
    .filter(|l| check_line(l) == 0)
    .collect::<Vec<&String>>();
  let mut line_scores = valid_lines
    .iter()
    .map(|l| line_score(l))
    .collect::<Vec<u64>>();
  line_scores.sort();

  println!(
    "Result part2: {:>10} | elapsed time: {:.2?}",
    line_scores[line_scores.len() / 2],
    before.elapsed()
  );
}

fn line_score(line: &str) -> u64 {
  let mut stack = Vec::new();
  for c in line.chars() {
    match c {
      '{' | '(' | '[' | '<' => stack.push(c),
      '}' | ')' | ']' | '>' => if stack.is_empty() || !brackets_match(stack.pop().unwrap(), c) {},
      _ => panic!(),
    }
  }
  stack.iter().rev().fold(0, |sum, c| {
    let closing_bracket = get_closing_backet(*c);
    sum * 5 + completion_score(closing_bracket)
  })
}

fn completion_score(c: char) -> u64 {
  match c {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4,
    _ => panic!(),
  }
}
