fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let data = parse_file(filename);
  let low_points = part1(&data);
  part2(&data, &low_points);
  println!("Total elapsed time: {:.2?}", before.elapsed());
}

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<Vec<u64>> {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let data = buf
    .lines()
    .map(|l| l.unwrap().chars().map(|e| e as u64 - '0' as u64).collect())
    .collect::<Vec<Vec<u64>>>();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  data
}

fn part1(data: &Vec<Vec<u64>>) -> Vec<(usize, usize)> {
  let before = std::time::Instant::now();
  let mut risk_level = 0;
  let mut low_points = Vec::new();
  for v in 0..data.len() {
    for h in 0..data[v].len() {
      let height = data[v][h];
      if (v == 0 || data[v - 1][h] > height)
        && (v == data.len() - 1 || data[v + 1][h] > height)
        && (h == 0 || data[v][h - 1] > height)
        && (h == data[v].len() - 1 || data[v][h + 1] > height)
      {
        risk_level += height + 1;
        low_points.push((h, v));
      }
    }
  }
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    risk_level,
    before.elapsed()
  );
  low_points
}

fn part2(_data: &Vec<Vec<u64>>, low_points: &Vec<(usize, usize)>) -> () {
  let before = std::time::Instant::now();
  low_points.fold(0, |size |)
  println!(
    "Result part2: {:>10} | elapsed time: {:.2?}",
    0,
    before.elapsed()
  );
}
