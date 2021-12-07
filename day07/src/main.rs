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

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<i64> {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let data = buf
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .split(',')
    .map(|e| e.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  data
}

fn part1(data: &Vec<i64>) -> () {
  let before = std::time::Instant::now();
  let mut crabs = data.clone();
  crabs.sort();
  let median = data[data.len() / 2];
  let mut fuel = 0;
  for crab in crabs {
    fuel += (crab - median).abs();
  }
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    fuel,
    before.elapsed()
  );
}

// the mean seems to be very close from the expected results so I just iterate from there
fn part2(data: &Vec<i64>) -> () {
  let before = std::time::Instant::now();
  let mut crabs = data.clone();
  crabs.sort();
  let mean = (crabs.iter().sum::<i64>() as f64 / crabs.len() as f64).round() as i64;
  let mean_fuel = fuel_cost(mean, &crabs);
  let mean_plus_one_fuel = fuel_cost(mean + 1, &crabs);
  let direction = if mean_plus_one_fuel < mean_fuel {
    1
  } else {
    -1
  };
  let mut previous_fuel = mean_fuel;
  let mut current_fuel = fuel_cost(mean + direction, &crabs);
  let mut iteration = direction + 1;
  while previous_fuel > current_fuel {
    previous_fuel = current_fuel;
    current_fuel = fuel_cost(mean + iteration * direction, &crabs);
    iteration += 1;
  }
  println!(
    "Result part2: {:>10} | elapsed time: {:.2?}",
    previous_fuel,
    before.elapsed()
  );
}

fn triangle_number(iteration: i64) -> i64 {
  iteration * (iteration + 1) / 2
}

fn fuel_cost(target: i64, crabs: &Vec<i64>) -> i64 {
  crabs.iter().fold(0, |cost, crab| {
    cost + triangle_number((crab - target).abs())
  })
}
