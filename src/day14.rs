use std::collections::HashMap;

const INPUT: &'static str = include_str!("../inputs/day14.txt");

pub(crate) fn run() {
    println!("day 14, output 1: {}", parse1(INPUT, 10));
    println!("day 14, output 2: {}", parse1(INPUT, 40));
}

fn parse1(input: &str, steps: usize) -> usize {
    let (mut pair_counts, mapping) = parse_input(input);

    for _ in 1..=steps {
        let mut new_counts: HashMap<Pair, usize> = HashMap::new();

        for (init, (out1, out2)) in &mapping {
            if pair_counts.contains_key(init) {
                let initial_count = pair_counts[init];
                let count = new_counts.entry(out1.clone()).or_default();
                *count += initial_count;
                let count = new_counts.entry(out2.clone()).or_default();
                *count += initial_count;
            }
        }

        pair_counts = new_counts;
    }

    get_max_minus_min(pair_counts)
}

fn get_max_minus_min(pair_counts: HashMap<Pair, usize>) -> usize {
    let mut letter_counts: HashMap<char, usize> = HashMap::new();
    for (pair, count) in pair_counts {
        *letter_counts.entry(pair.0 .0).or_default() += count;
        *letter_counts.entry(pair.0 .1).or_default() += count;
    }

    let mut max = usize::MIN;
    let mut min = usize::MAX;
    for (_, count) in letter_counts {
        let count = (count as f64 / 2.).ceil() as usize;
        if count < min {
            min = count
        } else if count > max {
            max = count
        }
    }

    max - min
}

fn parse_input(input: &str) -> (HashMap<Pair, usize>, HashMap<Pair, (Pair, Pair)>) {
    let (template, insertions) = input.split_once("\n\n").unwrap();
    let mut counts: HashMap<Pair, usize> = HashMap::new();
    let mut mapping: HashMap<Pair, (Pair, Pair)> = HashMap::new();
    let initial_pairs: Vec<Pair> = template
        .as_bytes()
        .windows(2)
        .map(|x| Pair((x[0] as char, x[1] as char)))
        .collect();

    for pair in initial_pairs {
        let count = counts.entry(pair).or_default();
        *count += 1;
    }

    insertions.lines().for_each(|line| {
        let (left, right) = line.split_once(" -> ").unwrap();
        let left: Vec<char> = left.chars().collect();
        let right: Vec<char> = right.chars().collect();

        let pair_init = Pair((left[0], left[1]));
        let pair_left = Pair((left[0], right[0]));
        let pair_right = Pair((right[0], left[1]));

        mapping.insert(pair_init, (pair_left, pair_right));
    });

    (counts, mapping)
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Pair((char, char));

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS, 10), 1588);
    }

    #[test]
    fn second() {
        assert_eq!(parse1(INPUTS, 40), 2188189693529);
    }
}
