use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day12.txt");

pub(crate) fn run() {
    println!("day 12, output 1: {}", parse1(INPUT));
    println!("day 12, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> usize {
    let mut graph: Graph = Graph::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();

        graph.insert(a, b);
    }

    let visited: HashSet<Cave> = HashSet::new();
    count_paths(&graph, Cave::Start, visited)
}

fn parse2(input: &str) -> usize {
    let mut graph: Graph = Graph::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();

        graph.insert(a, b);
    }

    let visited: HashSet<Cave> = HashSet::new();
    count_paths2(&graph, Cave::Start, visited, false)
}

fn count_paths(graph: &Graph, current: Cave, mut visited: HashSet<Cave>) -> usize {
    if current == Cave::End {
        return 1;
    } else if current.is_small() && visited.contains(&current) {
        return 0;
    }

    if current.is_small() {
        visited.insert(current.clone());
    }

    graph
        .get(&current)
        .unwrap()
        .into_iter()
        .map(|next| count_paths(graph, next.clone(), visited.clone()))
        .sum()
}

fn count_paths2(
    graph: &Graph,
    current: Cave,
    mut visited: HashSet<Cave>,
    mut used_second: bool,
) -> usize {
    if current == Cave::End {
        return 1;
    } else if current.is_small() && visited.contains(&current) && used_second {
        return 0;
    }

    used_second |= current.is_small() && visited.contains(&current);

    if current.is_small() {
        visited.insert(current.clone());
    }

    graph
        .get(&current)
        .unwrap()
        .into_iter()
        .map(|next| count_paths2(graph, next.clone(), visited.clone(), used_second))
        .sum()
}

struct Graph {
    map: HashMap<Cave, Vec<Cave>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, from: Cave, to: Cave) {
        let edges = self.map.entry(from.clone()).or_default();
        if !to.is_start() {
            edges.push(to.clone());
        }

        let edges = self.map.entry(to).or_default();
        if !from.is_start() {
            edges.push(from);
        }
    }

    fn get(&self, key: &Cave) -> Option<&Vec<Cave>> {
        self.map.get(key)
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
enum Cave {
    Start,
    End,
    Small(String),
    Large(String),
}

impl Cave {
    fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }

    fn is_small(&self) -> bool {
        matches!(self, Self::Small(_))
    }
}

impl FromStr for Cave {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "start" {
            Ok(Self::Start)
        } else if s == "end" {
            Ok(Self::End)
        } else if s.chars().all(|c| c.is_ascii_lowercase()) {
            Ok(Self::Small(s.to_owned()))
        } else {
            Ok(Self::Large(s.to_owned()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 226);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 3509);
    }
}
