const INPUT: &'static str = include_str!("../inputs/day3.txt");

pub(crate) fn run() {
    let prod = gamma_eps_prod(INPUT);
    println!("day 3, output 1: {}", prod);

    let lsr = life_support_rating(INPUT);
    println!("day 3, output 2: {}", lsr);
}

fn gamma_eps_prod(input: &str) -> i32 {
    let mut size = 0;
    let numbers: Vec<usize> = input
        .lines()
        .map(|line| {
            if size == 0 {
                size = line.len();
            }

            usize::from_str_radix(line, 2).unwrap()
        })
        .collect();

    let mut counts = vec![0; size];
    let n_lines = numbers.len();

    for mut num in numbers {
        let mut index = 0;
        while num != 0 {
            counts[index] += num & 1;
            index += 1;
            num >>= 1;
        }
    }

    let gamma = counts
        .iter()
        .map(|count| if *count >= (n_lines / 2) { 1 } else { 0 })
        .rfold(0, |acc, x| (acc << 1) | x);

    let all_ones = 2i32.pow(size as u32) - 1;
    let epsilon = gamma ^ all_ones; // Epsilon just invert of gamma

    epsilon * gamma
}

fn life_support_rating(input: &str) -> i32 {
    let mut size = 0;
    let numbers: Vec<usize> = input
        .lines()
        .map(|line| {
            if size == 0 {
                size = line.len();
            }

            usize::from_str_radix(line, 2).unwrap()
        })
        .collect();

    let mut oxygen_numbers = numbers.clone();
    for index in 0.. {
        let n_ones: usize = oxygen_numbers
            .iter()
            .map(|x| (x >> (size - index - 1)) & 1)
            .sum();

        let keep_ones = n_ones >= oxygen_numbers.len() - n_ones;

        oxygen_numbers = oxygen_numbers
            .clone()
            .into_iter()
            .filter(|x| {
                if keep_ones {
                    x >> (size - index - 1) & 1 == 1
                } else {
                    x >> (size - index - 1) & 1 == 0
                }
            })
            .collect();

        if oxygen_numbers.len() == 1 {
            break;
        }
    }

    let mut co2_numbers = numbers.clone();
    for index in 0.. {
        let n_ones: usize = co2_numbers
            .iter()
            .map(|x| (x >> (size - index - 1)) & 1)
            .sum();

        let keep_ones = n_ones < co2_numbers.len() - n_ones;

        co2_numbers = co2_numbers
            .clone()
            .into_iter()
            .filter(|x| {
                if keep_ones {
                    x >> (size - index - 1) & 1 == 1
                } else {
                    x >> (size - index - 1) & 1 == 0
                }
            })
            .collect();

        if co2_numbers.len() == 1 {
            break;
        }
    }

    oxygen_numbers[0] as i32 * co2_numbers[0] as i32
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    #[test]
    fn first() {
        assert_eq!(gamma_eps_prod(INPUTS), 198);
    }

    #[test]
    fn second() {
        assert_eq!(life_support_rating(INPUTS), 230);
    }
}
