use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day5.txt");

pub(crate) fn run() {
    let res = parse1(INPUT);
    println!("day 5, output 1: {}", res);

    let res = parse2(INPUT);
    println!("day 5, output 2: {}", res);
}

fn parse1(input: &str) -> usize {
    let mut counts: HashMap<(i32, i32), usize> = HashMap::new();
    let lines: Vec<Line> = input.lines().map(|line| line.parse().unwrap()).collect();

    for line in lines.into_iter() {
        match line {
            Line::HLine { y, xmin, xmax } => {
                for x in xmin..xmax + 1 {
                    match counts.get_mut(&(x, y)) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            counts.insert((x, y), 1);
                        }
                    }
                }
            }
            Line::VLine { x, ymin, ymax } => {
                for y in ymin..ymax + 1 {
                    match counts.get_mut(&(x, y)) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            counts.insert((x, y), 1);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    counts.into_iter().filter(|(_, count)| *count > 1).count()
}

fn parse2(input: &str) -> usize {
    let mut counts: HashMap<(i32, i32), usize> = HashMap::new();
    let lines: Vec<Line> = input.lines().map(|line| line.parse().unwrap()).collect();

    for line in lines.into_iter() {
        match line {
            Line::HLine { y, xmin, xmax } => {
                for x in xmin..xmax + 1 {
                    match counts.get_mut(&(x, y)) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            counts.insert((x, y), 1);
                        }
                    }
                }
            }
            Line::VLine { x, ymin, ymax } => {
                for y in ymin..ymax + 1 {
                    match counts.get_mut(&(x, y)) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            counts.insert((x, y), 1);
                        }
                    }
                }
            }
            Line::DLine {
                mut x,
                mut y,
                size,
                up,
            } => {
                for _ in 0..size {
                    match counts.get_mut(&(x, y)) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            counts.insert((x, y), 1);
                        }
                    }

                    x += 1;
                    if up {
                        y += 1
                    } else {
                        y -= 1
                    }
                }
            }
        }
    }

    counts.into_iter().filter(|(_, count)| *count > 1).count()
}

enum Line {
    HLine { y: i32, xmin: i32, xmax: i32 },
    VLine { x: i32, ymin: i32, ymax: i32 },
    DLine { x: i32, y: i32, size: i32, up: bool },
}

impl FromStr for Line {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();

        let p1: Vec<i32> = start.split(",").map(|val| val.parse().unwrap()).collect();
        let p2: Vec<i32> = end.split(",").map(|val| val.parse().unwrap()).collect();

        if p1[0] == p2[0] {
            Ok(Self::VLine {
                x: p1[0],
                ymin: p1[1].min(p2[1]),
                ymax: p1[1].max(p2[1]),
            })
        } else if p1[1] == p2[1] {
            Ok(Self::HLine {
                y: p1[1],
                xmin: p1[0].min(p2[0]),
                xmax: p1[0].max(p2[0]),
            })
        } else {
            let p1_is_left = p1[0] < p2[0];

            Ok(Self::DLine {
                x: if p1_is_left { p1[0] } else { p2[0] },
                y: if p1_is_left { p1[1] } else { p2[1] },
                size: (p2[0] - p1[0]).abs() + 1,
                up: if p1_is_left {
                    p1[1] < p2[1]
                } else {
                    p2[1] < p1[1]
                },
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 5);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 12);
    }
}
