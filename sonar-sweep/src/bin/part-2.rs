use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    // Read all lines to buffer
    while stdin.read_line(&mut buffer)? != 0 {}

    // Cast to integers
    let ints: Vec<i32> = buffer
        .lines()
        .flat_map(|line| line.parse::<i32>()) // ignores Err variant from Result of str.parse
        .collect();

    // Replace with 3 measurement sliding windows
    let sliding_window: Vec<i32> = ints.windows(3).map(|w| w.iter().sum()).collect();

    // Do the difference
    let sum = sliding_window
        .windows(2)
        .map(|w| if w[0] < w[1] { 1 } else { 0 })
        .sum::<i32>();

    println!("{:?}", sum);
    Ok(())
}
