// const INPUT: &'static str = include_str!("../inputs/day4.txt");
const INPUT: &'static str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

pub(crate) fn run() {
    let res = parse1(INPUT);
    println!("day 3, output 1: {}", res);

    // let res = parse2(INPUT);
    // println!("day 3, output 2: {}", res);
}

fn parse1(input: &str) -> i32 {
    let (numbers, boards) = input.split_once("\n\n").unwrap();
    let numbers: Vec<i32> = numbers.split(",").map(|x| x.parse().unwrap()).collect();

    let boards: Vec<Vec<Vec<i32>>> = boards
        .split("\n\n")
        .map(|board| {
            println!("board: {:?}", board);
            board
                .split("\n")
                .map(|row| {
                    row.split_whitespace()
                        .map(|col| col.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    return 0;
}

fn parse2(input: &str) -> () {}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";
    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 4512);
    }

    // #[test]
    // fn second() {
    //     assert_eq!(parse2(INPUTS), 230);
    // }
}
