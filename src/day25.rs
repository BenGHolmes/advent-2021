use std::collections::HashMap;
use std::fmt;

const INPUT: &'static str = include_str!("../inputs/day25.txt");

pub(crate) fn run() {
    println!("day 25, output 1: {}", parse1(INPUT));
}

fn parse1(input: &str) -> usize {
    let mut sim = Simulation::from_str(input);
    for step in 1usize.. {
        if matches!(sim.step(), Status::Done) {
            return step;
        }
    }

    unreachable!();
}

enum SeaCucumber {
    Right,
    Down
}

enum Status {
    Running,
    Done,
}

/// Position is (row,col) with row increasing down and col increasing right
struct Simulation {
   cucumbers: HashMap<(usize,usize), SeaCucumber>,
   height: usize,
   width: usize,
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                match self.cucumbers.get(&(row,col)) {
                    None => {write!(f, ".");}
                    Some(SeaCucumber::Right) => {write!(f, ">");}
                    Some(SeaCucumber::Down) => {write!(f, "v");}
                }
            }
            write!(f,"\n");
        };
        write!(f,"")
    }
}

impl Simulation {
    fn from_str(s: &str) -> Self {
        let mut height = s.lines().count();
        let mut width = s.len() / height;
        let mut cucumbers = HashMap::new();
        
        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if matches!(c, '>' | 'v') {
                    cucumbers.insert((row,col), if c == '>' {SeaCucumber::Right} else {SeaCucumber::Down});
                }
            }
        }

        Self  {cucumbers, height, width}
    }

    fn step(&mut self) -> Status {
        let mut moves_right = vec![];
        let mut moves_down = vec![];
        
        for (&(row,col), typ) in &self.cucumbers {
            if matches!(typ, SeaCucumber::Right) {
                let new_pos = (row,(col+1) % self.width);
                if !self.cucumbers.contains_key(&new_pos) {
                    moves_right.push((row,col));
                }
            }
        }

        for &(row,col) in &moves_right {
            self.cucumbers.remove(&(row,col));
            self.cucumbers.insert((row,(col+1) % self.width), SeaCucumber::Right);
        }

        for (&(row,col), typ) in &self.cucumbers {
            if matches!(typ, SeaCucumber::Down) {
                let new_pos = ((row+1)%self.height, col);
                if !self.cucumbers.contains_key(&new_pos) {
                    moves_down.push((row,col));
                }
            }
        }

        if moves_right.len() == 0 && moves_down.len() == 0 {
            Status::Done
        } else {
            for &(row,col) in &moves_down {
                self.cucumbers.remove(&(row,col));
                self.cucumbers.insert(((row+1)%self.height, col), SeaCucumber::Down);

            }
            Status::Running
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const input: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn parse() {
        let sim = Simulation::from_str(input);
        assert_eq!(sim.height, 9);
        assert_eq!(sim.width, 10);
        assert_eq!(format!("{}",sim), format!("{}\n", input));
    }

    #[test]
    fn steps() {
        let expected = "....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v
";

        let mut sim = Simulation::from_str(input);
        sim.step();


        // sim.step();
        
        assert_eq!(format!("{}",sim), expected);
    }

    #[test]
    fn first() {
        assert_eq!(parse1(input), 58);
    }
}
