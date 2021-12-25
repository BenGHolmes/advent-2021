use std::collections::HashSet;

const INPUT: &'static str = include_str!("../inputs/day20.txt");

pub(crate) fn run() {
    println!("day 20, output 1: {}", parse1(INPUT));
    println!("day 20, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> usize {
    let mut image = InfImage::from_str(input);
    image.enhance();
    image.enhance();
    image.count_lit()
}

fn parse2(input: &str) -> usize {
    let mut image = InfImage::from_str(input);
    for i in 1..=50 {
        image.enhance();
    }

    image.count_lit()
}

#[derive(Debug)]
struct InfImage {
    background: bool,
    algo: HashSet<u16>,
    image: Vec<Vec<bool>>,
}

impl InfImage {
    fn from_str(s: &str) -> Self {
        let (replacements, image) = s.split_once("\n\n").unwrap();
        let algo = replacements
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(|(i, _)| i as u16)
            .collect();

        let image: Vec<Vec<bool>> = image
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        Self {
            background: false,
            algo,
            image,
        }
    }

    fn enhance(&mut self) {
        let mut new_image: Vec<Vec<bool>> =
            vec![vec![false; self.image.len() + 2]; self.image[0].len() + 2];
        for row in -1i32..self.image.len() as i32 + 1 {
            for col in -1i32..self.image[0].len() as i32 + 1 {
                let encoding = self.encode(row, col);
                if self.algo.contains(&encoding) {
                    new_image[(row + 1) as usize][(col + 1) as usize] = true;
                }
            }
        }

        self.image = new_image;

        if self.background {
            self.background = self.algo.contains(&511);
        } else {
            self.background = self.algo.contains(&0);
        }
    }

    fn encode(&self, row: i32, col: i32) -> u16 {
        let mut encoding = 0;
        for r in row - 1..=row + 1 {
            for c in col - 1..=col + 1 {
                let this_bit = if self.is_background(r, c) {
                    self.background
                } else {
                    self.image[r as usize][c as usize]
                };

                encoding = (encoding << 1) | if this_bit { 1 } else { 0 };
            }
        }

        encoding
    }

    fn is_background(&self, row: i32, col: i32) -> bool {
        let n_rows = self.image.len() as i32;
        let n_cols = self.image[0].len() as i32;

        row < 0 || row >= n_rows || col < 0 || col >= n_cols
    }

    fn count_lit(&self) -> usize {
        self.image
            .iter()
            .map(|row| row.into_iter().filter(|&&lit| lit).count())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn encoding() {
        let image = InfImage::from_str(INPUTS);
        assert_eq!(image.encode(2, 2), 34);

        let image = InfImage::from_str("\n\n###\n###\n###");
        assert_eq!(image.encode(1, 1), 511);
        assert_eq!(image.encode(-100, -100), 0);
        assert_eq!(image.encode(-1, -1), 1);
        assert_eq!(image.encode(3, 3), 256);
    }

    #[test]
    fn algo() {
        let algo_line = INPUTS.lines().next().unwrap();
        let image = InfImage::from_str(INPUTS);

        for (i, c) in algo_line.chars().enumerate() {
            if c == '#' {
                assert!(image.algo.contains(&(i as u16)));
            }
        }
    }

    #[test]
    fn enhance() {
        let mut image = InfImage::from_str(INPUTS);
        image.enhance();

        let expected: Vec<Vec<bool>> = ".##.##.
#..#.#.
##.#..#
####..#
.#..##.
..##..#
...#.#."
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        assert_eq!(expected, image.image);
    }

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 35);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 3351);
    }
}
