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

fn parse_file(filename: impl AsRef<std::path::Path>) -> Instructions {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let data = buf.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
  let mut data_split = data.split(|l| l.is_empty());
  let polymer_template_string = data_split.next().unwrap()[0].clone();
  let mut polymer_template = HashMap::new();
  for i in 0..polymer_template_string.len() - 1 {
    let key = (
      polymer_template_string.chars().nth(i).unwrap(),
      polymer_template_string.chars().nth(i + 1).unwrap(),
    );
    *polymer_template.entry(key).or_insert(0) += 1;
  }
  let pair_insertions = data_split
    .next()
    .unwrap()
    .iter()
    .map(|l| {
      let mut iter = l.split(" -> ");
      let mut key = iter.next().unwrap().chars();
      (
        (key.next().unwrap(), key.next().unwrap()),
        iter.next().unwrap().chars().next().unwrap(),
      )
    })
    .collect();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  Instructions {
    polymer_template: polymer_template,
    pair_insertions: pair_insertions,
    last_letter: polymer_template_string
      .chars()
      .nth(polymer_template_string.len() - 1)
      .unwrap(),
  }
}

type Polymer = HashMap<(char, char), u64>;
type Insertions = HashMap<(char, char), char>;
struct Instructions {
  polymer_template: Polymer,
  pair_insertions: Insertions,
  last_letter: char,
}

fn part1(data: &Instructions) {
  let before = std::time::Instant::now();
  let mut polymer = data.polymer_template.clone();
  for _ in 0..10 {
    polymer = step(&polymer, &data.pair_insertions);
  }
  println!(
    "Result part1: {:>13} | elapsed time: {:.2?}",
    score(&polymer, data.last_letter),
    before.elapsed()
  );
}

fn step(polymer: &Polymer, pair_insertions: &Insertions) -> Polymer {
  let mut new_polymer = Polymer::new();
  for entry in polymer {
    let pair = entry.0;
    let occurences = entry.1;
    if pair_insertions.contains_key(&pair) {
      let new_char = pair_insertions[&pair];
      *new_polymer.entry((pair.0, new_char)).or_insert(0) += *occurences;
      *new_polymer.entry((new_char, pair.1)).or_insert(0) += *occurences;
    } else {
      *new_polymer.entry(*pair).or_insert(0) += *occurences;
    }
  }
  new_polymer
}

fn score(polymer: &Polymer, last_letter: char) -> u64 {
  let mut occurences = polymer.iter().fold(HashMap::new(), |mut occurence, e| {
    let key = e.0;
    *occurence.entry(key.0).or_insert(0) += e.1;
    occurence
  });
  *occurences.entry(last_letter).or_insert(0) += 1;
  let mut v = occurences
    .iter()
    .map(|e| (*e.0, *e.1))
    .collect::<Vec<(char, u64)>>();
  v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

  let most_common = occurences.iter().max_by_key(|e| e.1).unwrap();
  let least_common = occurences.iter().min_by_key(|e| e.1).unwrap();

  most_common.1 - least_common.1
}

fn part2(data: &Instructions) {
  let before = std::time::Instant::now();
  let mut polymer = data.polymer_template.clone();
  for _ in 0..40 {
    polymer = step(&polymer, &data.pair_insertions);
  }
  println!(
    "Result part2: {:>13} | elapsed time: {:.2?}",
    score(&polymer, data.last_letter),
    before.elapsed()
  );
}
