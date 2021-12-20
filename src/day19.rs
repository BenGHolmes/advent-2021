use ndarray::prelude::*;
use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("../inputs/day19.txt");
const UNIQUE_ROTS: [Rotation; 24] = [
    Rotation((0, 0, 0)),
    Rotation((0, 0, 1)),
    Rotation((0, 0, 2)),
    Rotation((0, 0, 3)),
    Rotation((0, 1, 0)),
    Rotation((0, 1, 1)),
    Rotation((0, 1, 2)),
    Rotation((0, 1, 3)),
    Rotation((0, 2, 0)),
    Rotation((0, 2, 1)),
    Rotation((0, 2, 2)),
    Rotation((0, 2, 3)),
    Rotation((0, 3, 0)),
    Rotation((0, 3, 1)),
    Rotation((0, 3, 2)),
    Rotation((0, 3, 3)),
    Rotation((1, 0, 0)),
    Rotation((1, 0, 1)),
    Rotation((1, 0, 2)),
    Rotation((1, 0, 3)),
    Rotation((1, 2, 0)),
    Rotation((1, 2, 1)),
    Rotation((1, 2, 2)),
    Rotation((1, 2, 3)),
];

pub(crate) fn run() {
    let (max_dist, beacon_count) = parse(INPUT);
    println!("day 19, output 1: {}", beacon_count);
    println!("day 19, output 2: {}", max_dist);
}

fn parse(input: &str) -> (i32, usize) {
    let mut lines = input.split("\n\n");
    let mut master = Scanner::from_str(lines.next().unwrap());
    let mut others: Vec<Scanner> = lines.map(|report| Scanner::from_str(report)).collect();

    let mut positions = vec![Point::new(0, 0, 0)];

    while others.len() > 0 {
        println!("{} Scanners remaining!", others.len());
        let mut merged = None;
        for (idx, other) in others.iter().enumerate() {
            if let Some(orientation) = master.is_match(&other) {
                positions.push(orientation.position);
                master.merge(&other, &orientation);
                merged = Some(idx);
                break;
            }
        }

        match merged {
            Some(idx) => {
                others.remove(idx);
            }
            _ => {
                panic!("No matches!");
            }
        }
    }

    let mut max_manhattan = 0;
    for i in 0..positions.len() - 1 {
        for j in i..positions.len() {
            max_manhattan = max_manhattan.max(positions[i].manhattan(&positions[j]));
        }
    }

    (max_manhattan, master.report.len())
}

#[derive(Debug)]
struct Scanner {
    orientation: Option<Orientation>,
    report: HashSet<Point>,
}

impl Scanner {
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines();
        lines.next(); // Skip header

        let mut report = HashSet::new();
        for line in lines {
            report.insert(Point::from_str(line));
        }

        Scanner {
            orientation: None,
            report,
        }
    }

    fn is_match(&self, other: &Self) -> Option<Orientation> {
        let mut votes: HashMap<Orientation, usize> = HashMap::new();
        for point_a in &self.report {
            for point_b in &other.report {
                for rotation in UNIQUE_ROTS {
                    let position = point_a.sub(&point_b.rotate90(&rotation));
                    let this_orientation = Orientation { position, rotation };
                    let vote_count = votes.entry(this_orientation.clone()).or_default();

                    *vote_count += 1;

                    if *vote_count == 12 {
                        return Some(this_orientation);
                    }
                }
            }
        }

        None
    }

    fn merge(&mut self, other: &Self, other_pos: &Orientation) {
        let Point {
            x: x0,
            y: y0,
            z: z0,
        } = other_pos.position;

        let new_report: HashSet<Point> = other
            .report
            .iter()
            .map(|point| {
                let Point { x, y, z } = point.rotate90(&other_pos.rotation);
                Point {
                    x: x0 + x,
                    y: y0 + y,
                    z: z0 + z,
                }
            })
            .collect();

        self.report.extend(new_report);
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Orientation {
    position: Point,
    rotation: Rotation,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Rotation((usize, usize, usize));

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let vals: Vec<i32> = s.split(",").map(|val| val.parse().unwrap()).collect();
        Point {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        }
    }

    fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }

    fn rotate90(&self, &Rotation((nx, ny, nz)): &Rotation) -> Self {
        let cos = [1, 0, -1, 0];
        let sin = [0, 1, 0, -1];

        let (cx, sx) = (cos[nx], sin[nx]);
        let (cy, sy) = (cos[ny], sin[ny]);
        let (cz, sz) = (cos[nz], sin[nz]);

        let rx = array![[1, 0, 0], [0, cx, -sx], [0, sx, cx]];
        let ry = array![[cy, 0, sy], [0, 1, 0], [-sy, 0, cy]];
        let rz = array![[cz, -sz, 0], [sz, cz, 0], [0, 0, 1]];

        let Point { x, y, z } = *self;

        let vec = array![x, y, z];
        let res = rx.dot(&ry).dot(&rz).dot(&vec).to_vec();

        Point::new(res[0] as i32, res[1] as i32, res[2] as i32)
    }

    fn sub(&self, other: &Self) -> Self {
        let Point {
            x: x1,
            y: y1,
            z: z1,
        } = self;
        let Point {
            x: x2,
            y: y2,
            z: z2,
        } = other;

        Point {
            x: x1 - x2,
            y: y1 - y2,
            z: z1 - z2,
        }
    }

    fn manhattan(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn rotate() {
        let pt = Point::new(1, 2, 3);
        let expected_coords = [
            (1, 2, 3),
            (-2, 1, 3),
            (-1, -2, 3),
            (2, -1, 3),
            (3, 2, -1),
            (3, 1, 2),
            (3, -2, 1),
            (3, -1, -2),
            (-1, 2, -3),
            (2, 1, -3),
            (1, -2, -3),
            (-2, -1, -3),
            (-3, 2, 1),
            (-3, 1, -2),
            (-3, -2, -1),
            (-3, -1, 2),
            (1, -3, 2),
            (-2, -3, 1),
            (-1, -3, -2),
            (2, -3, -1),
            (-1, 3, 2),
            (2, 3, 1),
            (1, 3, -2),
            (-2, 3, -1),
        ];
        for idx in 0..24 {
            let rot = &UNIQUE_ROTS[idx];
            let res = pt.rotate90(rot);

            let (x, y, z) = expected_coords[idx];
            let expected = Point::new(x, y, z);
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn is_match() {
        let input = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390";

        let scanners: Vec<Scanner> = input
            .split("\n\n")
            .map(|report| Scanner::from_str(report))
            .collect();

        match scanners[0].is_match(&scanners[1]) {
            Some(_) => (),
            None => assert!(false),
        };
    }

    #[test]
    fn example() {
        assert_eq!(parse(TEST_INPUT), (3621, 79));
    }
}
