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

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<Vec<u64>> {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let data = buf
    .lines()
    .map(|l| l.unwrap().chars().map(|c| c as u64 - '0' as u64).collect())
    .collect::<Vec<Vec<u64>>>();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  data
}

type Pos = (usize, usize);

fn part1(data: &Vec<Vec<u64>>) {
  let before = std::time::Instant::now();
  let mut octopodes = data.clone();
  let mut alight_count = 0;
  for _ in 0..100 {
    alight_count += step(&mut octopodes);
  }
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    alight_count,
    before.elapsed()
  );
}

fn part2(data: &Vec<Vec<u64>>) {
  let before = std::time::Instant::now();
  let mut octopodes = data.clone();
  let mut alight_count = 0;
  let mut step_count = 0;
  while alight_count as usize != octopodes.len() * octopodes[0].len() {
    alight_count = step(&mut octopodes);
    step_count += 1;
  }
  println!(
    "Result part2: {:>10} | elapsed time: {:.2?}",
    step_count,
    before.elapsed()
  );
}

fn step(octopodes: &mut Vec<Vec<u64>>) -> u64 {
  let mut alight_octopodes = increment_all_octopuses(octopodes);
  let mut alight_count = alight_octopodes.len() as u64;
  while !alight_octopodes.is_empty() {
    let alight_octopus = alight_octopodes.pop().unwrap();
    alight_count += increment_neighbours(octopodes, &mut alight_octopodes, alight_octopus);
  }
  reset_octopodes(octopodes);
  alight_count
}

fn reset_octopodes(octopodes: &mut Vec<Vec<u64>>) {
  for v in 0..octopodes.len() {
    for h in 0..octopodes[v].len() {
      if octopodes[v][h] > 9 {
        octopodes[v][h] = 0;
      }
    }
  }
}

fn increment_all_octopuses(octopodes: &mut Vec<Vec<u64>>) -> Vec<Pos> {
  let mut alight_octopodes = Vec::new();
  for v in 0..octopodes.len() {
    for h in 0..octopodes[v].len() {
      octopodes[v][h] += 1;
      if octopodes[v][h] > 9 {
        alight_octopodes.push((h, v))
      }
    }
  }
  alight_octopodes
}

fn increment_neighbours(
  octopodes: &mut Vec<Vec<u64>>,
  alight_octopodes: &mut Vec<Pos>,
  alight_octopus: Pos,
) -> u64 {
  let mut alight_count = 0;
  for v in -1..=1 {
    for h in -1..=1 {
      if alight_octopus.0 as i64 + h as i64 >= 0
        && alight_octopus.1 as i64 + v as i64 >= 0
        && (alight_octopus.0 as i64 + h as i64) < (octopodes[alight_octopus.1].len() as i64)
        && (alight_octopus.1 as i64 + v as i64) < (octopodes.len() as i64)
      {
        alight_count += increment_octopus(
          octopodes,
          alight_octopodes,
          (
            (alight_octopus.0 as i64 + h) as usize,
            (alight_octopus.1 as i64 + v) as usize,
          ),
        );
      }
    }
  }
  alight_count
}

fn increment_octopus(
  octopodes: &mut Vec<Vec<u64>>,
  alight_octopodes: &mut Vec<Pos>,
  octopus: Pos,
) -> u64 {
  octopodes[octopus.1][octopus.0] += 1;
  if octopodes[octopus.1][octopus.0] == 10 {
    alight_octopodes.push(octopus);
    1
  } else {
    0
  }
}
