const INPUT: &'static str = include_str!("../inputs/day1.txt");

pub(crate) fn run() {
    let count = count1(INPUT);
    println!("day 1, output 1: {}", count);

    let count = count2(INPUT);
    println!("day 1, output 2: {}", count);
}

fn count1(input: &str) -> usize {
    // Cast to integers
    let ints: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    // Do the difference
    ints.windows(2).filter(|x| x[0] < x[1]).count()
}

fn count2(input: &str) -> usize {
    // Cast to integers
    let ints: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    // Replace with 3 measurement sliding windows
    let sliding_window: Vec<i32> = ints.windows(3).map(|x| x.iter().sum()).collect();

    // Do the difference
    sliding_window.windows(2).filter(|x| x[0] < x[1]).count()
}
