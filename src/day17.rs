const INPUT: &'static str = include_str!("../inputs/day17.txt");

pub(crate) fn run() {
    println!("day 17, output 1: {}", parse1(INPUT));
    println!("day 17, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> i32 {
    let target = Target::from_str(input);

    let x_max = target.x_max;
    let x_min = ((f64::sqrt(1. + 8. * target.x_min as f64) - 1.) / 2.).ceil() as i32;

    let y_min = target.y_min as i32;
    let y_max = if y_min > 0 {
        target.y_max as i32
    } else if y_min < 0 {
        (target.y_min.abs() - 1) as i32
    } else {
        i32::MAX
    };

    for y in (y_min..=y_max).rev() {
        for x in x_min..=x_max {
            if hits(x, y, &target) {
                return (1..=y).sum();
            }
        }
    }

    panic!("no solution");
}

fn parse2(input: &str) -> usize {
    let target = Target::from_str(input);

    let x_max = target.x_max;
    let x_min = ((f64::sqrt(1. + 8. * target.x_min as f64) - 1.) / 2.).ceil() as i32;

    let y_min = target.y_min as i32;
    let y_max = if y_min > 0 {
        target.y_max as i32
    } else if y_min < 0 {
        (target.y_min.abs() - 1) as i32
    } else {
        i32::MAX
    };

    let mut n_hits = 0;

    for y in (y_min..=y_max).rev() {
        for x in x_min..=x_max {
            if hits(x, y, &target) {
                n_hits += 1;
            }
        }
    }

    n_hits
}

fn hits(mut dx: i32, mut dy: i32, target: &Target) -> bool {
    let mut x = 0;
    let mut y = 0;

    loop {
        x += dx;
        y += dy;
        dx = (dx - 1).max(0);
        dy = dy - 1;

        if x <= target.x_max && x >= target.x_min && y <= target.y_max && y >= target.y_min {
            return true;
        }

        if x > target.x_max || y < target.y_min {
            return false;
        }
    }
}

struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Target {
    fn from_str(s: &str) -> Self {
        let ranges = s.split_once("x=").unwrap().1;
        let (x_range, remainder) = ranges.split_once(",").unwrap();
        let (x_min, x_max) = x_range.split_once("..").unwrap();
        let y_range = remainder.split_once("y=").unwrap().1;
        let (y_min, y_max) = y_range.split_once("..").unwrap();
        let x_min: i32 = x_min.parse().unwrap();
        let x_max: i32 = x_max.parse().unwrap();
        let y_min: i32 = y_min.parse().unwrap();
        let y_max: i32 = y_max.parse().unwrap();

        Target {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 45);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 112);
    }
}
