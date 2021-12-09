use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day8.txt");

pub(crate) fn run() {
    let res = parse1(INPUT);
    println!("day 8, output 1: {}", res);

    let res = parse2(INPUT);
    println!("day 8, output 2: {}", res);
}

fn parse1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, output) = line.split_once(" | ").unwrap();
            output
                .split_whitespace()
                .filter(|code| match code.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn parse2(input: &str) -> usize {
    let mut res = 0;

    input.lines().for_each(|line| {
        let (sig, out) = line.split_once(" | ").unwrap();
        let mut signal_patterns: Vec<Signal> =
            sig.split_whitespace().map(|s| s.parse().unwrap()).collect();

        let mut map: HashMap<Signal, usize> = HashMap::new();
        let mut encodings: Vec<Option<Signal>> = vec![None; 10];
        let mut remainder = vec![];
        for sig in signal_patterns {
            match sig.len() {
                2 => {
                    map.insert(sig, 1);
                    encodings[1] = Some(sig);
                }
                3 => {
                    map.insert(sig, 7);
                    encodings[7] = Some(sig);
                }
                4 => {
                    map.insert(sig, 4);
                    encodings[4] = Some(sig);
                }
                7 => {
                    map.insert(sig, 8);
                    encodings[8] = Some(sig);
                }
                _ => remainder.push(sig),
            }
        }

        signal_patterns = remainder;
        let mut remainder = vec![];
        for sig in signal_patterns {
            match sig.len() {
                6 => {
                    if sig.is_superset(&encodings[4].unwrap()) {
                        map.insert(sig, 9);
                        encodings[9] = Some(sig);
                    } else if sig.is_superset(&encodings[1].unwrap()) {
                        map.insert(sig, 0);
                        encodings[0] = Some(sig);
                    } else {
                        map.insert(sig, 6);
                        encodings[6] = Some(sig);
                    }
                }
                _ => remainder.push(sig),
            }
        }

        signal_patterns = remainder;
        for sig in signal_patterns {
            if sig.is_superset(&encodings[1].unwrap()) {
                map.insert(sig, 3);
            } else if sig.is_subset(&encodings[9].unwrap()) {
                map.insert(sig, 5);
            } else {
                map.insert(sig, 2);
            }
        }

        res += out
            .split_whitespace()
            .map(|s| {
                let sig: Signal = s.parse().unwrap();
                map.get(&sig).unwrap()
            })
            .fold(0, |acc, x| acc * 10 + x);
    });

    res
}

#[derive(Eq, Hash, Clone, Copy)]
struct Signal(usize);

impl Signal {
    fn len(&self) -> usize {
        let mut n_set = 0;
        for i in 0..7 {
            n_set += (self.0 >> i) & 1;
        }
        n_set
    }

    fn is_subset(&self, other: &Self) -> bool {
        (self.0 ^ other.0) & self.0 == 0
    }

    fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }
}

impl FromStr for Signal {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let encoding = s
            .chars()
            .map(|c| c.to_digit(26).unwrap() as usize - 10)
            .rfold(0, |acc, offset| acc | (1 << offset));

        Ok(Self(encoding))
    }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 26);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 61229);
    }
}
