use std::collections::HashMap;
use std::collections::HashSet;
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

fn parse_file(filename: impl AsRef<std::path::Path>) -> Labyrinth {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let data = buf
    .lines()
    .map(|l| {
      l.unwrap()
        .split('-')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    })
    .collect::<Vec<Vec<String>>>()
    .iter()
    .fold(HashMap::new(), |mut map, path| {
      add_to_labyrinth(&mut map, &path[0], &path[1]);
      map
    });
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  data
}

fn add_to_labyrinth(labyrinth: &mut Labyrinth, a: &String, b: &String) {
  if a != "end" && b != "start" {
    labyrinth
      .entry(a.clone())
      .or_insert(Vec::new())
      .push(b.clone());
  }
  if a != "start" && b != "end" {
    labyrinth
      .entry(b.clone())
      .or_insert(Vec::new())
      .push(a.clone());
  }
}

type Labyrinth = HashMap<String, Vec<String>>;

fn part1(data: &Labyrinth) {
  let before = std::time::Instant::now();
  let visited = HashSet::new();
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    visit(data, false, visited, "start"),
    before.elapsed()
  );
}

fn part2(data: &Labyrinth) {
  let before = std::time::Instant::now();
  let visited = HashSet::new();
  println!(
    "Result part2: {:>10} | elapsed time: {:.2?}",
    visit(data, true, visited, "start"),
    before.elapsed()
  );
}

fn visit<'a>(
  data: &'a Labyrinth,
  bonus_passage: bool,
  visited: HashSet<&'a str>,
  current: &'a str,
) -> u64 {
  if current == "end" {
    return 1;
  }
  let mut path_count = 0;
  let mut visited_mut = visited;
  visited_mut.insert(current);
  for destination in data.get(current).unwrap_or(&Vec::new()) {
    let valid_passage = (destination.chars().next().unwrap() as u64) < ('a' as u64)
      || !visited_mut.contains(destination.as_str());
    if valid_passage || bonus_passage {
      path_count += visit(
        data,
        bonus_passage && valid_passage,
        visited_mut.clone(),
        destination,
      );
    }
  }
  path_count
}
