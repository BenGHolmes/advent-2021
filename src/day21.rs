use std::collections::HashMap;

const INPUT: &'static str = include_str!("../inputs/day21.txt");
const TRANSITIONS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

pub(crate) fn run() {
    println!("day 21, output 1: {}", parse1(INPUT));
    println!("day 21, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut p1 = Player::from_str(lines.next().unwrap());
    let mut p2 = Player::from_str(lines.next().unwrap());

    let mut die = Die::new();

    loop {
        if p1.take_turn(&mut die) {
            return p2.score * die.rolls;
        }
        if p2.take_turn(&mut die) {
            return p1.score * die.rolls;
        }
    }
}

fn parse2(input: &str) -> usize {
    let mut realities = HashMap::new();
    realities.insert(Reality::from_str(input), 1);
    let mut p1_count = 0;
    let mut p2_count = 0;

    let mut p1_move = true;

    loop {
        let mut new_realities: HashMap<Reality, usize> = HashMap::new();
        for (this_reality, count) in realities {
            let mut new = this_reality.split(p1_move);
            new.values_mut().for_each(|x| *x *= count);

            for (reality, count) in new {
                if reality.p1.score >= 21 {
                    p1_count += count;
                } else if reality.p2.score >= 21 {
                    p2_count += count;
                } else {
                    let current_val = new_realities.entry(reality).or_default();
                    *current_val += count;
                }
            }
        }

        if new_realities.len() == 0 {
            break;
        }

        realities = new_realities;
        p1_move = !p1_move;
    }

    p1_count.max(p2_count)
}

struct Die {
    value: usize,
    rolls: usize,
}

impl Die {
    fn new() -> Self {
        Die { value: 1, rolls: 0 }
    }

    fn roll(&mut self) -> usize {
        let roll = self.value;
        self.value = if self.value == 100 { 1 } else { self.value + 1 };
        self.rolls += 1;
        roll
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn from_str(s: &str) -> Self {
        let position: usize = s.split_once(": ").unwrap().1.parse().unwrap();
        Player { position, score: 0 }
    }

    fn take_turn(&mut self, die: &mut Die) -> bool {
        let roll = die.roll() + die.roll() + die.roll();
        self.position += roll;
        while self.position > 10 {
            self.position -= 10
        }

        self.score += self.position;

        self.score >= 1000
    }

    fn step(&self, step: usize) -> Player {
        let Player { position, score } = self;
        let mut new_pos = position + step;
        while new_pos > 10 {
            new_pos -= 10
        }

        let new_score = score + new_pos;

        Player {
            position: new_pos,
            score: new_score,
        }
    }

    fn copy(&self) -> Player {
        let Player { position, score } = self;
        Player {
            position: *position,
            score: *score,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Reality {
    p1: Player,
    p2: Player,
}

impl Reality {
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines();
        let p1 = Player::from_str(lines.next().unwrap());
        let p2 = Player::from_str(lines.next().unwrap());

        Reality { p1, p2 }
    }

    fn split(&self, p1_move: bool) -> HashMap<Reality, usize> {
        let mut new_realities: HashMap<Reality, usize> = HashMap::new();

        let Reality { p1, p2 } = self;

        for (roll, count) in TRANSITIONS {
            let new_p1 = if p1_move { p1.step(roll) } else { p1.copy() };
            let new_p2 = if p1_move { p2.copy() } else { p2.step(roll) };

            let current_count = new_realities
                .entry(Reality {
                    p1: new_p1,
                    p2: new_p2,
                })
                .or_default();

            *current_count += count;
        }

        new_realities
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 739785);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 444356092776315);
    }
}
