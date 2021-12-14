const INPUT: &'static str = include_str!("../inputs/day13.txt");

pub(crate) fn run() {
    println!("day 13, output 1: {}", parse1(INPUT));
    println!("day 13, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> usize {
    let (paper, folds) = input.split_once("\n\n").unwrap();
    let mut paper = Paper::from_str(paper);
    let folds: Vec<Fold> = folds.lines().map(|line| Fold::from_str(line)).collect();

    paper.fold(&folds[0]);
    paper.count_dots()
}

fn parse2(input: &str) -> usize {
    let (paper, folds) = input.split_once("\n\n").unwrap();
    let mut paper = Paper::from_str(paper);
    let folds: Vec<Fold> = folds.lines().map(|line| Fold::from_str(line)).collect();

    for fold in folds {
        paper.fold(&fold);
    }
    for row in paper.dots.iter() {
        for col in row.iter() {
            if *col {
                print!("##");
            } else {
                print!("..");
            }
        }
        println!("");
    }

    0
}

struct Paper {
    dots: Vec<Vec<bool>>,
}

impl Paper {
    fn from_str(s: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut points: Vec<(usize, usize)> = Vec::new();

        for line in s.lines() {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            points.push((x, y));

            width = width.max(x + 1);
            height = height.max(y + 1);
        }

        let mut dots = vec![vec![false; width]; height];

        for (x, y) in points {
            dots[y][x] = true
        }

        Paper { dots }
    }

    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::X(pos) => {
                self.dots = self
                    .dots
                    .iter()
                    .map(|row| {
                        let mut new_row = vec![];
                        for idx in 0..*pos {
                            let mirror_val = row.get(2 * pos - idx).unwrap_or(&false);
                            new_row.push(*row.get(idx).unwrap() || *mirror_val);
                        }
                        new_row
                    })
                    .collect();
            }
            Fold::Y(pos) => {
                for row in 0..*pos {
                    let mirror_row = 2 * pos - row;
                    if mirror_row < self.dots.len() {
                        for col in 0..self.dots[0].len() {
                            self.dots[row][col] |= self.dots[mirror_row][col];
                        }
                    }
                }

                self.dots = self.dots[0..*pos].to_vec();
            }
        }
    }

    fn count_dots(&self) -> usize {
        self.dots
            .iter()
            .map(|row| row.iter().filter(|is_dot| **is_dot).count())
            .sum()
    }
}

enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn from_str(s: &str) -> Self {
        if let Some((_, coord)) = s.split_once("x=") {
            Self::X(coord.parse().unwrap())
        } else if let Some((_, coord)) = s.split_once("y=") {
            Self::Y(coord.parse().unwrap())
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 17);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 3509);
    }
}
