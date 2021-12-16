use std::collections::HashMap;
use std::collections::HashSet;

use priority_queue::DoublePriorityQueue;

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

type Neighbours = HashSet<Pos>;
struct Node {
  _pos: Pos,
  local_distance: u64,
  distance: Option<u64>,
  is_visited: bool,
}

fn dijkstra(map: &Map) -> u64 {
  let mut unvisited = DoublePriorityQueue::<Pos, u64>::new();
  let mut nodes = create_nodes(&map);
  let neighbours = create_neighbours(&map);
  let mut current: Pos = (0, 0);
  nodes.get_mut(&current).unwrap().distance = Some(0);
  let end: Pos = (map.last().unwrap().len() - 1, map.len() - 1);
  while current != end {
    let current_distance = nodes[&current].distance.unwrap();
    for neighbour in &neighbours[&current] {
      let neighbour_local_distance = nodes[&neighbour].local_distance;
      let new_distance = neighbour_local_distance + current_distance;
      if nodes[&neighbour].distance.is_none() || new_distance < nodes[&neighbour].distance.unwrap()
      {
        nodes.get_mut(&neighbour).unwrap().distance = Some(new_distance);
        if !nodes[&neighbour].is_visited {
          unvisited.push(*neighbour, nodes[neighbour].distance.unwrap());
        } else {
          unvisited.change_priority(neighbour, nodes[neighbour].distance.unwrap());
        }
      }
    }
    nodes.get_mut(&current).unwrap().is_visited = true;
    unvisited.remove(&current);
    current = *unvisited.peek_min().unwrap().0;
  }
  nodes.get_mut(&end).unwrap().is_visited = true;
  nodes.get_mut(&end).unwrap().distance.unwrap()
}

fn create_nodes(map: &Map) -> HashMap<Pos, Node> {
  let mut nodes = HashMap::<Pos, Node>::new();
  for v in 0..map.len() {
    for h in 0..map[v].len() {
      let pos = (h, v);
      nodes.insert(
        pos,
        Node {
          _pos: pos,
          local_distance: map[v][h],
          distance: None,
          is_visited: false,
        },
      );
    }
  }
  nodes
}

fn create_neighbours(map: &Map) -> HashMap<Pos, Neighbours> {
  let mut neighbours = HashMap::new();
  for v in 0..map.len() {
    for h in 0..map[v].len() {
      let pos = (h, v);
      neighbours.insert(pos, get_neighbours(map, &pos));
    }
  }
  neighbours
}

fn get_neighbours(map: &Map, pos: &Pos) -> Neighbours {
  let mut neighbours = Neighbours::new();
  if pos.0 > 0 {
    neighbours.insert((pos.0 - 1, pos.1));
  }
  if pos.1 > 0 {
    neighbours.insert((pos.0, pos.1 - 1));
  }
  if pos.0 < map[pos.1].len() - 1 {
    neighbours.insert((pos.0 + 1, pos.1));
  }
  if pos.1 < map.len() - 1 {
    neighbours.insert((pos.0, pos.1 + 1));
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
