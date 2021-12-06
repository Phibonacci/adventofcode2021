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

type Vent = Vec<Pos>;
type Pos = Vec<i32>;

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<Vent> {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let vents = buf
    .lines()
    .map(|l| {
      l.unwrap()
        .split(" -> ")
        .map(|c| {
          c.split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Pos>()
        })
        .collect::<Vent>()
    })
    .collect::<Vec<Vent>>();

  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  vents
}

fn part1(data: &Vec<Vent>) -> () {
  let before = std::time::Instant::now();
  let mut map = HashMap::new();
  for vent in data {
    if vent[0][0] == vent[1][0] {
      for i in std::cmp::min(vent[0][1], vent[1][1])..=std::cmp::max(vent[0][1], vent[1][1]) {
        let key = (i, vent[0][0]);
        let current_value = if map.contains_key(&key) { map[&key] } else { 0 };
        map.insert(key, current_value + 1);
      }
    } else if vent[0][1] == vent[1][1] {
      for i in std::cmp::min(vent[0][0], vent[1][0])..=std::cmp::max(vent[0][0], vent[1][0]) {
        let key = (vent[0][1], i);
        let current_value = if map.contains_key(&key) { map[&key] } else { 0 };
        map.insert(key, current_value + 1);
      }
    }
  }
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    map
      .iter()
      .fold(0, |acc, e| acc + if *e.1 >= 2 { 1 } else { 0 }),
    before.elapsed()
  );
}

fn part2(data: &Vec<Vent>) -> () {
  let before = std::time::Instant::now();
  let mut map = HashMap::new();
  for vent in data {
    if vent[0][0] == vent[1][0] {
      for i in std::cmp::min(vent[0][1], vent[1][1])..=std::cmp::max(vent[0][1], vent[1][1]) {
        let key = (i, vent[0][0]);
        let current_value = *map.entry(key).or_insert(0);
        map.insert(key, current_value + 1);
      }
    } else if vent[0][1] == vent[1][1] {
      for i in std::cmp::min(vent[0][0], vent[1][0])..=std::cmp::max(vent[0][0], vent[1][0]) {
        let key = (vent[0][1], i);
        let current_value = *map.entry(key).or_insert(0);
        map.insert(key, current_value + 1);
      }
    } else if (vent[0][1] - vent[1][1]).abs() == (vent[0][0] - vent[1][0]).abs() {
      let y_mod = if vent[0][1] < vent[1][1] { 1 } else { -1 };
      let x_mod = if vent[0][0] < vent[1][0] { 1 } else { -1 };
      for i in 0..=(vent[0][1] - vent[1][1]).abs() {
        let key = (vent[0][1] + y_mod * i, vent[0][0] + x_mod * i);
        let current_value = *map.entry(key).or_insert(0);
        map.insert(key, current_value + 1);
      }
    } else {
      panic!();
    }
  }
  for y in 0..10 {
    for x in 0..10 {
      let key = (y, x);
      if map.contains_key(&key) {
        print!("{}", map[&key]);
      } else {
        print!(".");
      }
    }
    println!();
  }
  println!(
    "Result part2: {:>10} | elapsed time: {:.2?}",
    map
      .iter()
      .fold(0, |acc, e| acc + if *e.1 >= 2 { 1 } else { 0 }),
    before.elapsed()
  );
}
