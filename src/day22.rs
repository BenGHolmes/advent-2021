use std::collections::HashSet;

const INPUT: &'static str = include_str!("../inputs/day22.txt");

pub(crate) fn run() {
    println!("day 22, output 1: {}", parse1(INPUT));
    println!("day 22, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> i64 {
    let mut res = HashSet::new();
    for line in input.lines() {
        match line.split_once(" ").unwrap() {
            ("on", coords) => {
                let new_cuboid = Cuboid::from_str(coords);
                if new_cuboid.is_small() {
                    res = new_cuboid.add_to_group(res);
                }
            }
            ("off", coords) => {
                let new_cuboid = Cuboid::from_str(coords);
                if new_cuboid.is_small() {
                    res = new_cuboid.sub_from_group(res);
                }
            }
            _ => unreachable!(),
        }
    }

    res.iter().map(|cuboid| cuboid.volume()).sum()
}

fn parse2(input: &str) -> i64 {
    let mut res = HashSet::new();

    for line in input.lines() {
        match line.split_once(" ").unwrap() {
            ("on", coords) => {
                let new_cuboid = Cuboid::from_str(coords);
                res = new_cuboid.add_to_group(res);
            }
            ("off", coords) => {
                let new_cuboid = Cuboid::from_str(coords);
                res = new_cuboid.sub_from_group(res);
            }
            _ => unreachable!(),
        }
    }

    res.iter().map(|cuboid| cuboid.volume()).sum()
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Cuboid {
    bottom_left: (i64, i64, i64),
    top_right: (i64, i64, i64),
}

impl Cuboid {
    fn from_str(s: &str) -> Self {
        let mut values = s.split(",");
        let x = values.next().unwrap();
        let y = values.next().unwrap();
        let z = values.next().unwrap();

        let (x_min, x_max) = x.split_once("=").unwrap().1.split_once("..").unwrap();
        let (y_min, y_max) = y.split_once("=").unwrap().1.split_once("..").unwrap();
        let (z_min, z_max) = z.split_once("=").unwrap().1.split_once("..").unwrap();

        Cuboid {
            bottom_left: (
                x_min.parse().unwrap(),
                y_min.parse().unwrap(),
                z_min.parse().unwrap(),
            ),
            top_right: (
                x_max.parse().unwrap(),
                y_max.parse().unwrap(),
                z_max.parse().unwrap(),
            ),
        }
    }

    fn is_small(&self) -> bool {
        self.bottom_left.0 >= -50
            && self.bottom_left.1 >= -50
            && self.bottom_left.2 >= -50
            && self.top_right.0 <= 50
            && self.top_right.1 <= 50
            && self.top_right.2 <= 50
    }

    fn add_to_group(&self, others: HashSet<Cuboid>) -> HashSet<Cuboid> {
        let mut res = self.sub_from_group(others);
        res.insert(self.clone());
        res
    }

    fn sub_from_group(&self, others: HashSet<Cuboid>) -> HashSet<Cuboid> {
        let mut res = HashSet::new();
        for other in others {
            res.extend(other.sub(&self));
        }

        res
    }

    fn sub(&self, other: &Cuboid) -> HashSet<Cuboid> {
        let mut res = HashSet::new();
        match self.intersect(other) {
            None => {
                res.insert(self.clone());
                res
            }
            Some(intersect) => {
                // All y and z for x outside the range of other
                let x_min = other.bottom_left.0;
                let x_max = other.top_right.0;
                if self.bottom_left.0 < x_min {
                    res.insert(Cuboid {
                        bottom_left: self.bottom_left,
                        top_right: (
                            (x_min - 1).min(self.top_right.0),
                            self.top_right.1,
                            self.top_right.2,
                        ),
                    });
                }
                if self.top_right.0 > x_max {
                    res.insert(Cuboid {
                        bottom_left: (
                            (x_max + 1).max(self.bottom_left.0),
                            self.bottom_left.1,
                            self.bottom_left.2,
                        ),
                        top_right: self.top_right,
                    });
                }

                // All z for x in range of other and y outside range of other
                let y_min = other.bottom_left.1;
                let y_max = other.top_right.1;
                if self.bottom_left.1 < y_min {
                    res.insert(Cuboid {
                        bottom_left: (
                            intersect.bottom_left.0,
                            self.bottom_left.1,
                            self.bottom_left.2,
                        ),
                        top_right: (
                            intersect.top_right.0,
                            (y_min - 1).min(self.top_right.1),
                            self.top_right.2,
                        ),
                    });
                }
                if self.top_right.1 > y_max {
                    res.insert(Cuboid {
                        bottom_left: (
                            intersect.bottom_left.0,
                            (y_max + 1).max(self.bottom_left.1),
                            self.bottom_left.2,
                        ),
                        top_right: (intersect.top_right.0, self.top_right.1, self.top_right.2),
                    });
                }

                // Z outside of range of other for x,y in range
                let z_min = other.bottom_left.2;
                let z_max = other.top_right.2;
                if self.bottom_left.2 < z_min {
                    res.insert(Cuboid {
                        bottom_left: (
                            intersect.bottom_left.0,
                            intersect.bottom_left.1,
                            self.bottom_left.2,
                        ),
                        top_right: (
                            intersect.top_right.0,
                            intersect.top_right.1,
                            (z_min - 1).min(self.top_right.2),
                        ),
                    });
                }
                if self.top_right.2 > z_max {
                    res.insert(Cuboid {
                        bottom_left: (
                            intersect.bottom_left.0,
                            intersect.bottom_left.1,
                            (z_max + 1).max(self.bottom_left.2),
                        ),
                        top_right: (
                            intersect.top_right.0,
                            intersect.top_right.1,
                            self.top_right.2,
                        ),
                    });
                }

                res
            }
        }
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        if self.bottom_left.0 > other.top_right.0
            || self.bottom_left.1 > other.top_right.1
            || self.bottom_left.2 > other.top_right.2
            || self.top_right.0 < other.bottom_left.0
            || self.top_right.1 < other.bottom_left.1
            || self.top_right.2 < other.bottom_left.2
        {
            return None;
        }
        Some(Cuboid {
            bottom_left: (
                self.bottom_left.0.max(other.bottom_left.0),
                self.bottom_left.1.max(other.bottom_left.1),
                self.bottom_left.2.max(other.bottom_left.2),
            ),
            top_right: (
                self.top_right.0.min(other.top_right.0),
                self.top_right.1.min(other.top_right.1),
                self.top_right.2.min(other.top_right.2),
            ),
        })
    }

    fn volume(&self) -> i64 {
        (self.top_right.0 - self.bottom_left.0 + 1)
            * (self.top_right.1 - self.bottom_left.1 + 1)
            * (self.top_right.2 - self.bottom_left.2 + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersect() {
        let a = Cuboid {
            bottom_left: (0, 0, 0),
            top_right: (2, 2, 2),
        };
        let b = Cuboid {
            bottom_left: (1, 1, 1),
            top_right: (3, 3, 3),
        };
        let intersect = Cuboid {
            bottom_left: (1, 1, 1),
            top_right: (2, 2, 2),
        };

        assert_eq!(a.intersect(&b).unwrap(), intersect);
        assert_eq!(b.intersect(&a).unwrap(), intersect);
    }

    #[test]
    fn sub_disjoint() {
        let a = Cuboid {
            bottom_left: (0, 0, 0),
            top_right: (1, 1, 1),
        };
        let left = Cuboid {
            bottom_left: (-2, 0, 0),
            top_right: (-1, 1, 1),
        };
        let right = Cuboid {
            bottom_left: (2, 0, 0),
            top_right: (3, 1, 1),
        };
        let in_front = Cuboid {
            bottom_left: (0, 2, 0),
            top_right: (1, 3, 1),
        };
        let behind = Cuboid {
            bottom_left: (0, -2, 0),
            top_right: (1, -1, 1),
        };
        let above = Cuboid {
            bottom_left: (0, 0, 2),
            top_right: (1, 1, 3),
        };
        let below = Cuboid {
            bottom_left: (0, 0, -2),
            top_right: (1, 1, -1),
        };

        assert_eq!(a.sub(&left).iter().next().unwrap(), &a);
        assert_eq!(a.sub(&right).iter().next().unwrap(), &a);
        assert_eq!(a.sub(&in_front).iter().next().unwrap(), &a);
        assert_eq!(a.sub(&behind).iter().next().unwrap(), &a);
        assert_eq!(a.sub(&above).iter().next().unwrap(), &a);
        assert_eq!(a.sub(&below).iter().next().unwrap(), &a);
    }

    #[test]
    fn sub_planar() {
        let a = Cuboid {
            bottom_left: (0, 0, 0),
            top_right: (1, 1, 1),
        };
        let left = Cuboid {
            bottom_left: (0, 0, 0),
            top_right: (0, 1, 1),
        };
        let right = Cuboid {
            bottom_left: (1, 0, 0),
            top_right: (1, 1, 1),
        };
        let front = Cuboid {
            bottom_left: (0, 0, 0),
            top_right: (1, 0, 1),
        };
        let back = Cuboid {
            bottom_left: (0, 1, 0),
            top_right: (1, 1, 1),
        };
        let top = Cuboid {
            bottom_left: (0, 0, 1),
            top_right: (1, 1, 1),
        };
        let bottom = Cuboid {
            bottom_left: (0, 0, 0),
            top_right: (1, 1, 0),
        };
        assert_eq!(a.sub(&left).iter().next().unwrap(), &right);
        assert_eq!(a.sub(&right).iter().next().unwrap(), &left);
        assert_eq!(a.sub(&front).iter().next().unwrap(), &back);
        assert_eq!(a.sub(&back).iter().next().unwrap(), &front);
        assert_eq!(a.sub(&top).iter().next().unwrap(), &bottom);
        assert_eq!(a.sub(&bottom).iter().next().unwrap(), &top);
    }

    #[test]
    fn sub_enclosed() {
        let on = Cuboid {
            bottom_left: (0, 0, 0),
            top_right: (2, 2, 2),
        };
        let off = Cuboid {
            bottom_left: (1, 1, 1),
            top_right: (1, 1, 1),
        };

        let mut expected = HashSet::new();
        for cuboid in [
            Cuboid {
                bottom_left: (0, 0, 0),
                top_right: (0, 2, 2),
            },
            Cuboid {
                bottom_left: (2, 0, 0),
                top_right: (2, 2, 2),
            },
            Cuboid {
                bottom_left: (1, 0, 0),
                top_right: (1, 0, 2),
            },
            Cuboid {
                bottom_left: (1, 2, 0),
                top_right: (1, 2, 2),
            },
            Cuboid {
                bottom_left: (1, 1, 0),
                top_right: (1, 1, 0),
            },
            Cuboid {
                bottom_left: (1, 1, 2),
                top_right: (1, 1, 2),
            },
        ] {
            expected.insert(cuboid);
        }

        assert_eq!(on.sub(&off), expected);
    }

    #[test]
    fn sub() {
        let a = Cuboid::from_str("x=-33..18,y=-35..11,z=-49..2");
        let b = Cuboid::from_str("x=-14..32,y=5..49,z=-42..5");
        let set = a.sub(&b);
        let mut expected = HashSet::new();
        expected.insert(Cuboid::from_str("x=-33..-15,y=-35..11,z=-49..2"));
        expected.insert(Cuboid::from_str("x=-14..18,y=-35..4,z=-49..2"));
        expected.insert(Cuboid::from_str("x=-14..18,y=5..11,z=-49..-43"));

        assert_eq!(set, expected);
    }

    #[test]
    fn add_to_group() {
        let a = Cuboid::from_str("x=0..1,y=0..0,z=0..0");
        let b = Cuboid::from_str("x=2..3,y=0..0,z=0..0");
        let c = Cuboid::from_str("x=1..2,y=0..0,z=0..0");

        let mut merged = HashSet::new();
        merged.insert(a.clone());
        merged.insert(b.clone());
        merged = c.add_to_group(merged);

        let mut expected = HashSet::new();
        expected.insert(Cuboid::from_str("x=0..0,y=0..0,z=0..0"));
        expected.insert(Cuboid::from_str("x=3..3,y=0..0,z=0..0"));
        expected.insert(Cuboid::from_str("x=1..2,y=0..0,z=0..0"));

        assert_eq!(merged, expected);
        assert_eq!(merged.iter().map(|cube| cube.volume()).sum::<i64>(), 4);
    }

    #[test]
    fn volume() {
        let a = Cuboid::from_str("x=-33..18,y=-35..11,z=-49..2");
        let b = Cuboid::from_str("x=-14..32,y=5..49,z=-42..5");

        let mut collection = HashSet::new();
        collection.insert(a.clone());
        collection = b.add_to_group(collection);

        println!("A: {:?}", a);
        println!("B: {:?}", b);
        println!("A & B: {:?}", a.intersect(&b).unwrap());

        println!("\nA - B = ");
        for res in &collection {
            if res != &b {
                println!("\t{:?}: {}", res, res.volume());
            }
        }

        println!(
            "expected = {} + {} - {}",
            a.volume(),
            b.volume(),
            a.intersect(&b).unwrap().volume()
        );

        let expected_volume = a.volume() + b.volume() - a.intersect(&b).unwrap().volume();
        assert_eq!(
            collection.iter().map(|c| c.volume()).sum::<i64>(),
            expected_volume
        );
    }

    #[test]
    fn first() {
        let input = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";
        assert_eq!(parse1(input), 590784);
    }

    #[test]
    fn second() {
        let input = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
        assert_eq!(parse2(input), 2758514936282235);
    }
}
