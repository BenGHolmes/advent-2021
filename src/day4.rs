use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day4.txt");

pub(crate) fn run() {
    let res = parse1(INPUT);
    println!("day 4, output 1: {}", res);

    let res = parse2(INPUT);
    println!("day 4, output 2: {}", res);
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<(i32, bool)>>,
}

impl Board {
    fn mark(&mut self, value: i32) {
        for row in self.board.iter_mut() {
            for (cell_val, is_marked) in row {
                if *cell_val == value {
                    *is_marked = true;
                    return;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> i32 {
        self.board.iter().fold(0, |acc, row| {
            acc + row.iter().fold(
                0,
                |acc, (value, marked)| if !(*marked) { acc + value } else { acc },
            )
        })
    }

    fn validate(&self) -> Option<i32> {
        for index in 0..self.board.len() {
            let row_complete = self.board[index].iter().all(|(_, marked)| *marked);
            let col_complete = self
                .board
                .iter()
                .map(|row| row[index])
                .all(|(_, marked)| marked);

            if row_complete || col_complete {
                return Some(self.sum_unmarked());
            }
        }

        return None;
    }
}

impl FromStr for Board {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: Vec<Vec<(i32, bool)>> = s
            .split("\n")
            .map(|row| {
                row.split_whitespace()
                    .map(|col| (col.parse().unwrap(), false))
                    .collect()
            })
            .collect();

        Ok(Self { board })
    }
}

fn parse1(input: &str) -> i32 {
    let (numbers, board_strs) = input.split_once("\n\n").unwrap();
    let numbers: Vec<i32> = numbers.split(",").map(|x| x.parse().unwrap()).collect();

    let mut boards: Vec<Board> = board_strs
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .collect();

    for num in numbers {
        for board in boards.iter_mut() {
            board.mark(num);
            if let Some(sum_unmarked) = board.validate() {
                return sum_unmarked * num;
            }
        }
    }

    panic!("No answer!");
}

fn parse2(input: &str) -> i32 {
    let (numbers, board_strs) = input.split_once("\n\n").unwrap();
    let numbers: Vec<i32> = numbers.split(",").map(|x| x.parse().unwrap()).collect();

    let mut boards: Vec<Board> = board_strs
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .collect();

    for num in numbers {
        if boards.len() == 1 {
            boards[0].mark(num);
            if let Some(sum_unmarked) = boards[0].validate() {
                return sum_unmarked * num;
            }
        } else {
            for board in boards.iter_mut() {
                board.mark(num);
            }
        }

        boards = boards
            .into_iter()
            .filter(|board| match board.validate() {
                None => true,
                _ => false,
            })
            .collect();
    }

    panic!("No answer!");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 4512);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 1924);
    }
}
