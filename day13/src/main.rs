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

fn parse_file(filename: impl AsRef<std::path::Path>) -> Instructions {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let data = buf.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
  let mut data_split = data.split(|l| l.is_empty());
  let mut paper = HashSet::new();
  for s in data_split.next().unwrap() {
    let mut split = s.split(',');
    let coordinates = (
      split.next().unwrap().parse::<u64>().unwrap(),
      split.next().unwrap().parse::<u64>().unwrap(),
    );
    paper.insert(coordinates);
  }
  let mut fold_instructions = Vec::new();
  for s in data_split.next().unwrap() {
    let mut split = s.split('=');
    let axe = split.next().unwrap().chars().last().unwrap();
    let coordinate = split.next().unwrap().parse::<u64>().unwrap();
    if axe == 'x' {
      fold_instructions.push((coordinate, 0));
    } else {
      fold_instructions.push((0, coordinate));
    }
  }
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  Instructions {
    paper,
    fold_instructions,
  }
}

type Pos = (u64, u64);
type Paper = HashSet<Pos>;
type FoldInstructions = Vec<Pos>;
struct Instructions {
  paper: Paper,
  fold_instructions: FoldInstructions,
}

fn part1(data: &Instructions) {
  let before = std::time::Instant::now();
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    fold_paper(data, 1).len(),
    before.elapsed()
  );
}

fn fold_paper(data: &Instructions, fold_count: usize) -> Paper {
  let mut paper = data.paper.clone();
  for fold_index in 0..fold_count {
    let fold_instruction = data.fold_instructions[fold_index];
    paper = fold_once(&paper, &fold_instruction);
  }
  paper
}

fn fold_once(paper: &Paper, fold_instruction: &Pos) -> Paper {
  let mut next_paper = HashSet::new();
  for pos in paper {
    if fold_instruction.0 > 0 {
      if pos.0 == fold_instruction.0 {
        continue;
      } else if pos.0 > fold_instruction.0 {
        let x = fold_coordinate(pos.0, fold_instruction.0);
        next_paper.insert((x, pos.1));
      } else {
        next_paper.insert(*pos);
      }
    } else {
      if pos.1 == fold_instruction.1 {
        continue;
      } else if pos.1 > fold_instruction.1 {
        let y = fold_coordinate(pos.1, fold_instruction.1);
        next_paper.insert((pos.0, y));
      } else {
        next_paper.insert(*pos);
      }
    }
  }
  next_paper
}

fn fold_coordinate(coordinate: u64, fold_point: u64) -> u64 {
  coordinate - (coordinate - fold_point) * 2
}

fn part2(data: &Instructions) {
  let before = std::time::Instant::now();
  let paper = fold_paper(data, data.fold_instructions.len());
  println!(
    "Result part2: {} | elapsed time: {:.2?}",
    format_paper(&paper),
    before.elapsed()
  );
}

fn format_paper(paper: &Paper) -> String {
  let x_max = paper
    .iter()
    .fold(None, |max, b| {
      if max.is_none() || max.unwrap() < b.0 {
        Some(b.0)
      } else {
        max
      }
    })
    .unwrap();
  let y_max = paper
    .iter()
    .fold(None, |max, b| {
      if max.is_none() || max.unwrap() < b.1 {
        Some(b.1)
      } else {
        max
      }
    })
    .unwrap();
  let mut result = String::new();
  for y in 0..=y_max {
    result.push('\n');
    for x in 0..=x_max {
      if paper.contains(&(x, y)) {
        result.push('#');
      } else {
        result.push('.');
      }
    }
  }
  result
}
