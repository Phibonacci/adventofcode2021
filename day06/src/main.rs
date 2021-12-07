use std::collections::HashMap;

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

fn parse_file(filename: impl AsRef<std::path::Path>) -> HashMap<u64, u64> {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let fishes = buf
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .split(',')
    .map(|e| e.parse::<i64>().unwrap())
    .fold(HashMap::new(), |mut map, e| {
      *map.entry(e.abs() as u64).or_insert(0) += 1;
      map
    });
  println!(
    "{}",
    fishes
      .iter()
      .fold(String::new(), |s, e| format!("{},{}:{}", s, e.1, e.0))
  );
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  fishes
}

fn part1(data: &HashMap<u64, u64>) -> () {
  let before = std::time::Instant::now();
  println!(
    "Result part1: {:>13} | elapsed time: {:.2?}",
    life(data, 80),
    before.elapsed()
  );
}

fn part2(data: &HashMap<u64, u64>) -> () {
  let before = std::time::Instant::now();
  println!(
    "Result part2: {:>13} | elapsed time: {:.2?}",
    life(data, 256),
    before.elapsed()
  );
}

fn life(data: &HashMap<u64, u64>, cycles: u64) -> u64 {
  let mut fishes = data.clone();
  let mut babies = HashMap::new();
  for days in 1..=cycles {
    let new_babies = *fishes.entry((6 + days) % 7).or_insert(0);
    let new_fishes = *babies.entry(days % 2).or_insert(0);
    *fishes.entry((6 + days) % 7).or_insert(0) += new_fishes;
    babies.insert(days % 2, new_babies);
  }
  fishes.iter().fold(0, |acc, e| acc + *e.1) + babies.iter().fold(0, |acc, e| acc + *e.1)
}
