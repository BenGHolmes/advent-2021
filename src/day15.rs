use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashSet, VecDeque};

const INPUT: &'static str = include_str!("../inputs/day15.txt");

pub(crate) fn run() {
    println!("day 15, output 1: {}", parse1(INPUT));
    println!("day 15, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    shortest_path(grid).expect("No path")
}

fn parse2(input: &str) -> u32 {
    let small_grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let n_rows = small_grid.len();
    let n_cols = small_grid[0].len();

    let mut grid = vec![vec![0; 5 * n_cols]; 5 * n_rows];
    for row in 0..5 * n_rows {
        for col in 0..5 * n_cols {
            let inc = (row / n_rows) + (col / n_cols);
            let mut val = small_grid[row % n_rows][col % n_cols] + inc as u32;
            if val > 9 {
                val -= 9;
            }
            grid[row][col] = val;
        }
    }

    shortest_path(grid).expect("no path")
}

fn shortest_path(grid: Vec<Vec<u32>>) -> Option<u32> {
    let max_row = grid.len() - 1;
    let max_col = grid[0].len() - 1;
    let goal = (max_row, max_col);

    let mut costs = vec![vec![u32::MAX; max_row + 1]; max_col + 1];

    let mut heap = BinaryHeap::new();

    costs[0][0] = 0;
    heap.push(State {
        cost: 0,
        pos: (0, 0),
    });

    while let Some(State { cost, pos }) = heap.pop() {
        // If we are at the goal, return
        if pos == goal {
            return Some(cost);
        }

        let (row, col) = pos;
        // Ignore if we've alread found a better way. This is because we don't update the costs in the heap
        if cost > costs[row][col] {
            continue;
        }

        for (next_row, next_col) in [
            (row.checked_sub(1).unwrap_or(0), col),
            (row + 1, col),
            (row, col.checked_sub(1).unwrap_or(0)),
            (row, col + 1),
        ] {
            if (next_row, next_col) == pos || next_row > max_row || next_col > max_col {
                continue;
            }

            let next_cost = cost + grid[next_row][next_col];

            if next_cost < costs[next_row][next_col] {
                heap.push(State {
                    cost: next_cost,
                    pos: (next_row, next_col),
                });
                costs[next_row][next_col] = next_cost;
            }
        }
    }

    None
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    pos: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 40);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 315);
    }
}
