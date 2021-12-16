use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::HashMap;

use priority_queue::PriorityQueue;

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

fn parse_file(filename: impl AsRef<std::path::Path>) -> Map {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let data = buf
    .lines()
    .map(|l| l.unwrap().chars().map(|c| c as u64 - '0' as u64).collect())
    .collect();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  data
}

type Map = Vec<Vec<u64>>;
type Pos = (usize, usize);

fn part1(data: &Map) {
  let before = std::time::Instant::now();
  println!(
    "Result part1: {:>10} | elapsed time: {:.2?}",
    dijkstra(data),
    before.elapsed()
  );
}

type Neighbours = Vec<Pos>;

#[derive(PartialEq, Eq, Hash)]
struct Distance {
  distance: u64,
}

impl PartialOrd for Distance {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.distance.partial_cmp(&self.distance)
  }
}

impl Ord for Distance {
  fn cmp(&self, other: &Self) -> Ordering {
    self.distance.cmp(&other.distance)
  }
}

fn dijkstra(map: &Map) -> u64 {
  let mut unvisited = PriorityQueue::<Pos, Distance>::new();
  let mut nodes = HashMap::new();
  let mut current: Pos = (0, 0);
  let mut current_distance = 0;
  nodes.insert(current, 0);
  let end: Pos = (map.last().unwrap().len() - 1, map.len() - 1);

  while current != end {
    for neighbour in &get_neighbours(map, &current) {
      let neighbour_local_distance = map[neighbour.1][neighbour.0];
      let new_distance = neighbour_local_distance + current_distance;
      let node_exists = nodes.contains_key(&neighbour);
      if !node_exists || new_distance < nodes[&neighbour] {
        if !node_exists {
          unvisited.push(
            *neighbour,
            Distance {
              distance: new_distance,
            },
          );
          nodes.insert(*neighbour, new_distance);
        } else {
          unvisited.change_priority(
            neighbour,
            Distance {
              distance: new_distance,
            },
          );
          *nodes.get_mut(&neighbour).unwrap() = new_distance;
        }
      }
    }
    let next = unvisited.pop().unwrap();
    current = next.0;
    current_distance = next.1.distance;
  }
  *nodes.get_mut(&end).unwrap()
}

fn get_neighbours(map: &Map, pos: &Pos) -> Neighbours {
  let mut neighbours = Neighbours::new();
  if pos.0 > 0 {
    neighbours.push((pos.0 - 1, pos.1));
  }
  if pos.1 > 0 {
    neighbours.push((pos.0, pos.1 - 1));
  }
  if pos.0 < map[pos.1].len() - 1 {
    neighbours.push((pos.0 + 1, pos.1));
  }
  if pos.1 < map.len() - 1 {
    neighbours.push((pos.0, pos.1 + 1));
  }
  neighbours
}

fn part2(data: &Map) {
  let before = std::time::Instant::now();
  let mut big_map = Vec::new();
  for v in 0..data.len() {
    let mut new_line = Vec::new();
    for i in 0..5 {
      new_line = [
        new_line,
        data[v].iter().map(|e| (e + i - 1) % 9 + 1).collect(),
      ]
      .concat();
    }
    big_map.push(new_line);
  }
  for i in 1..5 {
    for v in 0..data.len() {
      big_map.push(big_map[v].iter().map(|e| (e + i - 1) % 9 + 1).collect());
    }
  }
  println!(
    "Result part2: {:>10} | elapsed time: {:.2?}",
    dijkstra(&big_map),
    before.elapsed()
  );
}
