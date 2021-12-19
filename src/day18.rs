const INPUT: &'static str = include_str!("../inputs/day18.txt");

pub(crate) fn run() {
    println!("day 18, output 1: {}", parse1(INPUT));
    println!("day 18, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut result = SnailNum::from_str(lines.next().unwrap());

    for line in lines {
        let this_num = SnailNum::from_str(line);
        result.add(this_num);
    }

    result.magnitude()
}

fn parse2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let n_lines = lines.len();

    let mut max_magnitude = 0;

    for i in 0..n_lines {
        for j in 0..n_lines {
            if i == j {
                continue;
            }
            let mut a = SnailNum::from_str(lines[i]);
            let b = SnailNum::from_str(lines[j]);

            a.add(b);
            max_magnitude = max_magnitude.max(a.magnitude());
        }
    }

    max_magnitude
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct RegularNum {
    value: u32,
    depth: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct SnailNum {
    values: Vec<RegularNum>,
}

impl SnailNum {
    fn from_str(s: &str) -> Self {
        let mut values = vec![];

        let mut depth = 0;
        for c in s.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => continue,
                val => values.push(RegularNum {
                    value: val.to_digit(10).unwrap(),
                    depth,
                }),
            }
        }

        Self { values }
    }

    fn add(&mut self, mut other: Self) {
        self.values.iter_mut().for_each(|em| em.depth += 1);
        other.values.iter_mut().for_each(|em| em.depth += 1);
        self.values.extend(other.values);

        loop {
            if self.explode() || self.split() {
                continue;
            }

            break;
        }
    }

    fn explode(&mut self) -> bool {
        let n_values = self.values.len();

        for idx in 0..n_values - 1 {
            if self.values[idx].depth == 5 {
                if idx != 0 {
                    self.values[idx - 1].value += self.values[idx].value;
                }
                if idx + 2 < n_values {
                    self.values[idx + 2].value += self.values[idx + 1].value;
                }

                self.values[idx] = RegularNum {
                    depth: self.values[idx].depth - 1,
                    value: 0,
                };

                self.values.remove(idx + 1);

                return true;
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        let n_values = self.values.len();

        for idx in 0..n_values {
            let RegularNum { value, mut depth } = self.values[idx];
            if value > 9 {
                let left = value / 2;
                let right = (value + 1) / 2;
                depth += 1;

                self.values[idx].value = right;
                self.values[idx].depth = depth;
                self.values.insert(idx, RegularNum { value: left, depth });

                return true;
            }
        }

        false
    }

    fn magnitude(&self) -> u32 {
        let mut magnitudes = self.values.clone();
        loop {
            for idx in 0..magnitudes.len() - 1 {
                if magnitudes[idx].depth == magnitudes[idx + 1].depth {
                    let depth = magnitudes[idx].depth;
                    let l_val = magnitudes[idx].value;
                    let r_val = magnitudes[idx + 1].value;
                    magnitudes[idx] = RegularNum {
                        depth: depth - 1,
                        value: 3 * l_val + 2 * r_val,
                    };

                    magnitudes.remove(idx + 1);

                    break;
                }
            }

            if magnitudes.len() == 1 {
                return magnitudes[0].value;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let s = "[[9,[8,7]],5]";
        let sn = SnailNum::from_str(s);

        assert_eq!(
            sn,
            SnailNum {
                values: vec![
                    RegularNum { depth: 2, value: 9 },
                    RegularNum { depth: 3, value: 8 },
                    RegularNum { depth: 3, value: 7 },
                    RegularNum { depth: 1, value: 5 }
                ]
            }
        )
    }

    #[test]
    fn explode() {
        let mut s_init = SnailNum::from_str("[[[[[9,8],1],2],3],4]");
        let s_final = SnailNum::from_str("[[[[0,9],2],3],4]");

        assert!(s_init.explode());

        assert_eq!(s_init, s_final);
    }

    #[test]
    fn split() {
        let mut s_init = SnailNum {
            values: vec![
                RegularNum {
                    value: 15,
                    depth: 1,
                },
                RegularNum { value: 5, depth: 1 },
            ],
        };
        let s_final = SnailNum::from_str("[[7,8],5]");

        assert!(s_init.split());
        assert_eq!(s_init, s_final);
    }

    #[test]
    fn add() {
        let mut a = SnailNum::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = SnailNum::from_str("[1,1]");

        let c = SnailNum::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        a.add(b);

        assert_eq!(a, c);
    }

    #[test]
    fn magnitude() {
        for (input, magnitude) in [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ] {
            let num = SnailNum::from_str(input);
            assert_eq!(num.magnitude(), magnitude);
        }
    }

    #[test]
    fn first() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        assert_eq!(parse1(input), 4140);
    }

    #[test]
    fn second() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        assert_eq!(parse2(input), 3993);
    }
}
