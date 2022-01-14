use std::collections::HashSet;

const INPUT: &'static str = include_str!("../inputs/day24.txt");

pub(crate) fn run() {
    let mut model_generator = ModelGenerator::new(INPUT);
    println!("day 24, output 1: {}", model_generator.max());
    println!("day 24, output 2: {}", model_generator.min());
}

/// Each digit of the program undergoes 18 steps
///     inp w
///     mul x 0
///     add x z
///     mod x 26
///     div z <VALUE>
///     add x <VALUE>
///     eql x w
///     eql x 0
///     mul y 0
///     add y 25
///     mul y x
///     add y 1
///     mul z y
///     mul y 0
///     add y w
///     add y <VALUE>
///     mul y x
///     add z y
/// 
/// This can be reduced to the following rust code
/// ```rust
/// fn run(inp: i64, current_z: i64) -> i64 {
///     let mut w = inp;
///     let mut z = current_z
///     let mut x = z;
///     x %= 26;
///     z /= <VALUE>;
///     x += <VALUE>;
///     x = if x == w {1} else {0}
///     x = if x == 0 {1} else {0}
///     let mut y = 25;
///     y *= x;
///     y += 1;
///     z *= y;
///     y = w;
///     y += <VALUE>;
///     y *= x;
///     z += y;
/// }
/// ```
#[derive(Debug)]
struct ModelGenerator {
    // Stores (digit,z) pairs that didn't produce a valid answer
    bad_states: HashSet<(usize,i64)>,
    div_z: [i64;14],
    add_x: [i64;14],
    add_y: [i64;14],
}

impl ModelGenerator {
    fn new(prog: &str) -> Self {
        let mut div_z = [0;14];
        let mut add_x = [0;14];
        let mut add_y = [0;14];
       
        prog.split("inp w").skip(1).enumerate().for_each(|(digit, operations)| {
            let mut lines = operations.lines();
            div_z[digit] = lines.nth(4).unwrap().split(" ").nth(2).unwrap().parse().unwrap();
            add_x[digit] = lines.nth(0).unwrap().split(" ").nth(2).unwrap().parse().unwrap();
            add_y[digit] = lines.nth(9).unwrap().split(" ").nth(2).unwrap().parse().unwrap();
        });

        Self{bad_states: HashSet::new(), div_z, add_x, add_y}
    }

    fn max(&mut self) -> usize {
        self.search(0, 0, 0, (1..=9).rev().collect()).unwrap()
    }

    fn min(&mut self) -> usize {
        self.search(0, 0, 0, (1..=9).collect()).unwrap()
    }

    fn search(&mut self, depth: usize, mut current_num: usize, original_z: i64, digits: Vec<usize>) -> Option<usize> {
        // Break if we know this state is bad
        if self.bad_states.contains(&(depth, original_z)) || depth == 14 {
            return None;
        }

        current_num *= 10;

        for digit in digits.clone() {
            let mut w = digit as i64;
            let mut z = original_z;
            let mut x = z;
            x %= 26;
            z /= self.div_z[depth];
            x += self.add_x[depth];
            x = if x == w {1} else {0};
            x = if x == 0 {1} else {0};
            let mut y = 25;
            y *= x;
            y += 1;
            z *= y;
            y = w;
            y += self.add_y[depth];
            y *= x;
            z += y;

            if z == 0 && depth == 13 {
                return Some(current_num + digit);
            }

            if let Some(res) = self.search(depth+1, current_num+digit, z, digits.clone()) {
                return Some(res);
            }
        }

        self.bad_states.insert((depth, original_z));

        None
    }
}