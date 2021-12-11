const INPUT: &'static str = include_str!("../inputs/day10.txt");

pub(crate) fn run() {
    println!("day 10, output 1: {}", parse1(INPUT));
    println!("day 10, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            let mut stack: Vec<char> = vec![];
            let mut this_line_score = 0;

            for c in s.chars() {
                match c {
                    '<' | '{' | '(' | '[' => stack.push(c),
                    '>' => {
                        if stack.pop().unwrap() != '<' {
                            this_line_score = 25137;
                            break;
                        }
                    }
                    '}' => {
                        if stack.pop().unwrap() != '{' {
                            this_line_score = 1197;
                            break;
                        }
                    }
                    ')' => {
                        if stack.pop().unwrap() != '(' {
                            this_line_score = 3;
                            break;
                        }
                    }
                    ']' => {
                        if stack.pop().unwrap() != '[' {
                            this_line_score = 57;
                            break;
                        }
                    }
                    _ => panic!("invalid character!"),
                }
            }

            this_line_score
        })
        .sum()
}

fn parse2(input: &str) -> usize {
    let mut scores: Vec<usize> = vec![];
    input.lines().for_each(|s| {
        let mut stack = remainder(s);
        let mut score = 0;
        while !stack.is_empty() {
            match stack.pop().unwrap() {
                '(' => score = score * 5 + 1,
                '[' => score = score * 5 + 2,
                '{' => score = score * 5 + 3,
                '<' => score = score * 5 + 4,
                _ => {}
            }
        }

        if score > 0 {
            scores.push(score);
        }
    });

    scores.sort();
    return scores[scores.len() / 2];
}

fn remainder(s: &str) -> Vec<char> {
    let mut stack: Vec<char> = vec![];

    let keep = s.chars().all(|c| match c {
        '<' | '{' | '(' | '[' => {
            stack.push(c);
            true
        }
        '>' => stack.pop().unwrap() == '<',
        '}' => stack.pop().unwrap() == '{',
        ')' => stack.pop().unwrap() == '(',
        ']' => stack.pop().unwrap() == '[',
        _ => panic!("unexpected char"),
    });

    return if keep { stack } else { vec![] };
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 26397);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 288957);
    }
}
