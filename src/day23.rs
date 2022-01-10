use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

const INPUT: &'static str = include_str!("../inputs/day23.txt");

pub(crate) fn run() {
    println!("day 23, output 1: {}", parse1(INPUT));
    println!("day 23, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> usize {
    let amphipods = parse_amphipods(input);
    let initial_burrow = Burrow {
        hallway: [None; 11],
        rooms: [
            [Some(amphipods[0]), Some(amphipods[4])],
            [Some(amphipods[1]), Some(amphipods[5])],
            [Some(amphipods[2]), Some(amphipods[6])],
            [Some(amphipods[3]), Some(amphipods[7])],
        ],
    };

    solve(initial_burrow)
}

fn parse2(input: &str) -> usize {
    let amphipods = parse_amphipods(input);
    let initial_burrow = Burrow {
        hallway: [None; 11],
        rooms: [
            [
                Some(amphipods[0]),
                Some(Amphipod::D),
                Some(Amphipod::D),
                Some(amphipods[4]),
            ],
            [
                Some(amphipods[1]),
                Some(Amphipod::C),
                Some(Amphipod::B),
                Some(amphipods[5]),
            ],
            [
                Some(amphipods[2]),
                Some(Amphipod::B),
                Some(Amphipod::A),
                Some(amphipods[6]),
            ],
            [
                Some(amphipods[3]),
                Some(Amphipod::A),
                Some(Amphipod::C),
                Some(amphipods[7]),
            ],
        ],
    };

    solve(initial_burrow)
}

fn parse_amphipods(s: &str) -> Vec<Amphipod> {
    s.chars()
        .filter_map(|c| match c {
            'A' => Some(Amphipod::A),
            'B' => Some(Amphipod::B),
            'C' => Some(Amphipod::C),
            'D' => Some(Amphipod::D),
            _ => None,
        })
        .collect::<Vec<_>>()
}

fn solve<const R: usize>(initial_state: Burrow<R>) -> usize {
    let mut q = BinaryHeap::new();
    q.push(State {
        state: initial_state,
        energy: 0,
    });

    let mut scores: HashMap<Burrow<R>, usize> = HashMap::new();
    scores.insert(initial_state, 0);

    let goal_state = Burrow::<R>::goal();

    while let Some(State { state, energy }) = q.pop() {
        if state == goal_state {
            return energy;
        }

        let current_score = scores[&state];

        for (next_state, transition_cost) in state.transitions() {
            let next_score = current_score + transition_cost;
            if next_score < *scores.get(&next_state).unwrap_or(&usize::MAX) {
                scores.insert(next_state, next_score);
                q.push(State {
                    state: next_state,
                    energy: next_score + next_state.h_score(),
                });
            }
        }
    }

    unreachable!();
}

#[derive(PartialEq, Eq)]
struct State<const R: usize> {
    state: Burrow<R>,
    energy: usize,
}

impl<const R: usize> PartialOrd<Self> for State<R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const R: usize> Ord for State<R> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.energy.cmp(&other.energy).reverse()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl Amphipod {
    fn energy(&self) -> usize {
        10usize.pow(*self as u32)
    }

    fn target_room(&self) -> usize {
        *self as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Burrow<const R: usize> {
    hallway: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; R]; 4],
}

impl<const R: usize> Burrow<R> {
    fn goal() -> Self {
        Self {
            hallway: [None; 11],
            rooms: [
                [Some(Amphipod::A); R],
                [Some(Amphipod::B); R],
                [Some(Amphipod::C); R],
                [Some(Amphipod::D); R],
            ],
        }
    }

    // Heuristic for A*
    fn h_score(&self) -> usize {
        // Cost to move all amphipods to their target room from the hall
        let hall_to_room: usize = self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(current_x, space)| space.map(|amphipod| (current_x, amphipod)))
            .map(|(current_x, amphipod)| {
                let target_x = self.room_x(amphipod.target_room());
                let steps = abs_diff(current_x, target_x);
                steps * amphipod.energy()
            })
            .sum();

        // Cost to move all amphipods from current rom to target room
        let room_to_room: usize = self
            .rooms
            .iter()
            .enumerate()
            .flat_map(|(room_idx, room)| {
                let current_x = self.room_x(room_idx);
                room.iter()
                    .filter_map(move |space| space.map(|amphipod| (current_x, amphipod)))
                    .map(|(current_x, amphipod)| {
                        let target_x = self.room_x(amphipod.target_room());
                        let steps = abs_diff(current_x, target_x);
                        steps * amphipod.energy()
                    })
            })
            .sum();

        hall_to_room + room_to_room
    }

    fn transitions(&self) -> Vec<(Burrow<R>, usize)> {
        let mut transitions = self.into_hallway_transitions();
        transitions.extend(self.into_room_transitions());
        transitions
    }

    fn into_hallway_transitions(&self) -> Vec<(Burrow<R>, usize)> {
        self.rooms
            .iter()
            .enumerate()
            .filter(|(room_index, _)| !self.all_match(*room_index))
            .flat_map(|(room_index, room)| {
                let (idx, amphipod) = room
                    .iter()
                    .enumerate()
                    .find_map(|(idx, space)| space.map(|amphipod| (idx, amphipod)))
                    .unwrap();

                let current_x = self.room_x(room_index);

                self.empty_spaces(current_x)
                    .into_iter()
                    .filter(|target_x| !self.is_above_room(*target_x))
                    .map(move |target_x| {
                        let steps = idx + 1 + abs_diff(current_x, target_x);
                        let energy = steps * amphipod.energy();

                        let mut state = *self;
                        std::mem::swap(
                            &mut state.hallway[target_x],
                            &mut state.rooms[room_index][idx],
                        );
                        (state, energy)
                    })
            })
            .collect()
    }
    fn into_room_transitions(&self) -> Vec<(Burrow<R>, usize)> {
        self.hallway
            .iter()
            .enumerate()
            .filter_map(|(current_x, space)| space.map(|amphipod| (current_x, amphipod)))
            .filter_map(|(current_x, amphipod)| {
                let target_room = amphipod.target_room();
                if !self.all_match(target_room) {
                    // Room still contains other amphipods
                    return None;
                }

                let target_x = self.room_x(target_room);

                if !self.hall_is_clear(current_x, target_x) {
                    // Hallway blocked
                    return None;
                }

                let target_idx = self.rooms[target_room]
                    .iter()
                    .rposition(|space| space.is_none())
                    .unwrap();

                let steps = target_idx + 1 + abs_diff(current_x, target_x);
                let energy = steps * amphipod.energy();

                let mut state = *self;
                std::mem::swap(
                    &mut state.hallway[current_x],
                    &mut state.rooms[target_room][target_idx],
                );

                Some((state, energy))
            })
            .collect()
    }

    fn hall_is_clear(&self, start: usize, target: usize) -> bool {
        let slice = match start.cmp(&target) {
            Ordering::Equal => {
                return true;
            }
            Ordering::Less => &self.hallway[(start + 1)..=target],
            Ordering::Greater => &self.hallway[target..start],
        };

        slice.iter().all(|space| space.is_none())
    }

    fn room_x(&self, room_index: usize) -> usize {
        2 * room_index + 2
    }

    fn is_above_room(&self, idx: usize) -> bool {
        matches!(idx, 2 | 4 | 6 | 8)
    }

    fn all_match(&self, room_index: usize) -> bool {
        self.rooms[room_index].iter().all(|space| match space {
            None => true,
            Some(amphipod) => amphipod.target_room() == room_index,
        })
    }

    fn empty_spaces(&self, start: usize) -> Vec<usize> {
        let left_it = (0..start).rev().take_while(|x| self.hallway[*x].is_none());
        let right_it = ((start + 1)..self.hallway.len()).take_while(|x| self.hallway[*x].is_none());
        left_it.chain(right_it).collect()
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const input: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn first() {
        assert_eq!(parse1(input), 12521);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(input), 44169);
    }
}
