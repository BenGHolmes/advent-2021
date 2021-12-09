use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day6.txt");

pub(crate) fn run() {
    let res = parse1(INPUT, 80);
    println!("day 6, output 1: {}", res);

    let res = parse2(INPUT, 256);
    println!("day 6, output 2: {}", res);
}

fn parse1(input: &str, n_days: i32) -> usize {
    let mut pond: Vec<Fish> = input.split(",").map(|s| s.parse().unwrap()).collect();
    let mut new_fish: Vec<Fish> = Vec::new();
    for _ in 1..=n_days {
        for fish in pond.iter_mut() {
            if let Some(fish) = fish.step() {
                new_fish.push(fish);
            }
        }

        pond.append(&mut new_fish);
    }

    pond.len()
}

fn parse2(input: &str, n_days: i32) -> usize {
    let mut counts = [0; 9];

    input.split(",").for_each(|s| {
        let state: usize = s.parse().unwrap();
        counts[state] += 1;
    });

    for _ in 1..=n_days {
        let mut new_counts = [0; 9];
        for state in 1..=8 {
            new_counts[state - 1] = counts[state];
        }
        new_counts[8] = counts[0];
        new_counts[6] += counts[0];

        counts = new_counts;
    }

    counts.iter().sum()
}

#[derive(Copy, Clone, Debug)]
struct Fish(usize);

impl Fish {
    fn new(timer: usize) -> Self {
        Self(timer)
    }

    fn step(&mut self) -> Option<Fish> {
        if self.0 == 0 {
            self.0 = 6;
            Some(Fish::new(8))
        } else {
            self.0 -= 1;
            None
        }
    }
}

impl FromStr for Fish {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Fish(s.parse().unwrap()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "3,4,3,1,2";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS, 18), 26);
        assert_eq!(parse1(INPUTS, 80), 5934);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS, 256), 26984457539);
    }
}
