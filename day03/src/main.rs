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
  let content = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  content
}

fn part1(data: &Vec<String>) -> () {
  let before = std::time::Instant::now();
  let mut weights = vec![0 as i32; data[0].len()];
  for number in data {
    for i in 0..number.len() {
      match number.chars().nth(i).unwrap() {
        '0' => weights[i] -= 1,
        '1' => weights[i] += 1,
        _ => panic!(),
      }
    }
  }
  let mut gamma = 0;
  let mut epsilon = 0;
  for i in 0..weights.len() {
    gamma <<= 1;
    epsilon <<= 1;
    if weights[i] > 0 {
      gamma |= 0x1;
    } else {
      epsilon |= 0x1;
    }
  }
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    gamma * epsilon,
    before.elapsed()
  );
}

fn part2(data: &Vec<String>) -> () {
  let before = std::time::Instant::now();
  let ref_data = data.iter().map(|e| e).collect();
  let oxygen_generator = extract(1, &ref_data);
  let co2_scrubber = extract(-1, &ref_data);
  println!(
    "Result part2: {:>10} | elapsed time: {:.2?}",
    oxygen_generator * co2_scrubber,
    before.elapsed()
  );
}

fn extract<'a>(order: i32, data: &Vec<&'a String>) -> u32 {
  let mut zeros = Vec::new();
  let mut ones = Vec::new();
  let mut target = data.clone();
  for i in 0..data[0].len() {
    if target.len() == 1 {
      break;
    }
    for number in target {
      match number.chars().nth(i).unwrap() {
        '0' => zeros.push(number),
        '1' => ones.push(number),
        _ => panic!(),
      }
    }
    if order > 0 {
      if ones.len() >= zeros.len() {
        target = ones;
      } else {
        target = zeros;
      }
    } else {
      if zeros.len() <= ones.len() {
        target = zeros;
      } else {
        target = ones;
      }
    }
    zeros = Vec::new();
    ones = Vec::new();
  }
  assert_eq!(target.len(), 1);
  let mut value = 0;
  for c in target[0].chars() {
    value <<= 1;
    if c == '1' {
      value |= 0x1;
    }
  }
  value
}
