use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day7.txt");

pub(crate) fn run() {
    let res = parse1(INPUT);
    println!("day 7, output 1: {}", res);

    let res = parse2(INPUT);
    println!("day 7, output 2: {}", res);
}

fn parse1(input: &str) -> i32 {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    let pos: Vec<i32> = input
        .split(",")
        .map(|x| {
            let val = x.parse().unwrap();
            min = min.min(val);
            max = max.max(val);

            val
        })
        .collect();

    let mut gas = i32::MAX;
    for target in min..=max {
        gas = gas.min(pos.iter().map(|x| (target - x).abs()).sum());
    }

    gas
}

fn parse2(input: &str) -> i32 {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    let pos: Vec<i32> = input
        .split(",")
        .map(|x| {
            let val = x.parse().unwrap();
            min = min.min(val);
            max = max.max(val);

            val
        })
        .collect();

    let mut gas = i32::MAX;
    for target in min..=max {
        let cost = pos
            .iter()
            .map(|x| {
                let d = (target - x).abs();
                d * (d + 1) / 2
            })
            .sum();
        gas = gas.min(cost);
    }

    gas
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 37);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 168);
    }
}
