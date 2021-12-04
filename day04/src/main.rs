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

type Square = Vec<Vec<u32>>;
struct Bingo {
  draw: Vec<u32>,
  squares: Vec<Square>,
}

fn parse_file(filename: impl AsRef<std::path::Path>) -> Bingo {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let mut data = buf.lines();
  let draw = data
    .next()
    .unwrap()
    .unwrap()
    .split(',')
    .collect::<Vec<&str>>()
    .iter()
    .map(|e| e.parse::<u32>().unwrap())
    .collect::<Vec<u32>>();
  data.next();
  let squares = data
    .map(|l| l.unwrap())
    .collect::<Vec<String>>()
    .split(|l| l.is_empty())
    .map(|a| {
      a.iter()
        .map(|l| {
          l.split_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
        })
        .collect::<Square>()
    })
    .collect::<Vec<Square>>();

  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  Bingo { draw, squares }
}

fn part1(bingo: &Bingo) -> () {
  let before = std::time::Instant::now();
  for draw_count in bingo.squares[0].len()..bingo.draw.len() {
    for square in &bingo.squares {
      let value = check_square(square, &bingo.draw[0..draw_count]);
      if value > 0 {
        println!(
          "Result part1: {:>10} | elapsed time: {:.2?}",
          value,
          before.elapsed()
        );
        return;
      }
    }
  }
}

fn check_square(square: &Square, draws: &[u32]) -> u32 {
  let lines_value = check_lines(square, draws);
  if lines_value > 0 {
    return lines_value;
  }
  let column_value = check_columns(square, draws);
  return column_value;
}

fn check_lines(square: &Square, draws: &[u32]) -> u32 {
  for line in square {
    let mut found = true;
    for number in line {
      if !draws.contains(number) {
        found = false;
        break;
      }
    }
    if found {
      return calculate_score(square, draws);
    }
  }
  return 0;
}

fn check_columns(square: &Square, draws: &[u32]) -> u32 {
  for i in 0..square[0].len() {
    let mut found = true;
    for line in square {
      if !draws.contains(&line[i]) {
        found = false;
        break;
      }
    }
    if found {
      return calculate_score(square, draws);
    }
  }
  return 0;
}

fn calculate_score(square: &Square, draws: &[u32]) -> u32 {
  draws.last().unwrap()
    * square
      .iter()
      .map(|l| {
        l.iter()
          .fold(0, |acc, n| if !draws.contains(n) { acc + n } else { acc })
      })
      .sum::<u32>()
}

fn part2(bingo: &Bingo) -> () {
  let before = std::time::Instant::now();
  let mut squares = bingo.squares[0..bingo.squares.len()].to_vec();
  for draw_count in bingo.squares[0].len()..bingo.draw.len() {
    let mut to_remove = Vec::<usize>::new();
    for i in 0..squares.len() {
      let value = check_square(&squares[i], &bingo.draw[0..draw_count]);
      if value > 0 {
        if squares.len() > 1 {
          to_remove.push(i);
        } else {
          println!(
            "Result part1: {:>10} | elapsed time: {:.2?}",
            value,
            before.elapsed()
          );
          return;
        }
      }
    }
    to_remove.sort();
    for index in to_remove.into_iter().rev() {
      squares.remove(index);
    }
  }
}
