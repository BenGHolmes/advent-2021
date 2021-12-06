const INPUT: &'static str = include_str!("../inputs/day2.txt");

pub(crate) fn run() {
    let product = final_pos_prod(INPUT);
    println!("day 2, output 1: {}", product);

    let product = aim_pos_prod(INPUT);
    println!("day 2, output 2: {}", product);
}

fn final_pos_prod(input: &str) -> i32 {
    let mut depth = 0;
    let mut position = 0;

    input.lines().for_each(|line| {
        let x: Vec<&str> = line.split(" ").collect();
        match x[0] {
            "forward" => position += x[1].parse::<i32>().unwrap(),
            "down" => depth += x[1].parse::<i32>().unwrap(),
            "up" => depth -= x[1].parse::<i32>().unwrap(),
            _ => {}
        }
    });

    depth * position
}

fn aim_pos_prod(input: &str) -> i32 {
    let mut aim = 0;
    let mut pos = 0;
    let mut depth = 0;

    input.lines().for_each(|line| {
        let x: Vec<&str> = line.split(" ").collect();
        match x[0] {
            "forward" => {
                let val = x[1].parse::<i32>().unwrap();
                pos += val;
                depth += val * aim;
            }
            "down" => aim += x[1].parse::<i32>().unwrap(),
            "up" => aim -= x[1].parse::<i32>().unwrap(),
            _ => {}
        }
    });

    depth * pos
}
