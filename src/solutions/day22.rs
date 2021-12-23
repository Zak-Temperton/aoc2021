use regex::Regex;

use crate::solutions::day22::cube::Cube;

pub fn part1(text: &str) {
    let r = Regex::new(r"(?:(on|off)) x=(?:([\-\d]+))..(?:([\-\d]+)),y=(?:([\-\d]+))..(?:([\-\d]+)),z=(?:([\-\d]+))..(?:([\-\d]+))").unwrap();
    let mut cubes: Vec<Cube> = Vec::new();
    for line in text.lines().take(20) {
        let captures = r.captures(line).unwrap();
        let new_cube = Cube::from(captures);
        for cube in &mut cubes {
            cube.sub(&new_cube);
        }
        if new_cube.on {
            cubes.push(new_cube);
        }
    }
    println!("part1: {}", cubes.iter().fold(0, |a, c| a + c.volume()));
}

pub fn part2(text: &str) {
    let r = Regex::new(r"(?:(on|off)) x=(?:([\-\d]+))..(?:([\-\d]+)),y=(?:([\-\d]+))..(?:([\-\d]+)),z=(?:([\-\d]+))..(?:([\-\d]+))").unwrap();
    let mut cubes: Vec<Cube> = Vec::new();
    for line in text.lines() {
        let captures = r.captures(line).unwrap();
        let new_cube = Cube::from(captures);
        for cube in &mut cubes {
            cube.sub(&new_cube);
        }
        if new_cube.on {
            cubes.push(new_cube);
        }
    }
    println!("part2: {}", cubes.iter().fold(0, |a, c| a + c.volume()));
}

mod cube {
    use std::cmp;

    use regex::Captures;

    #[derive(Clone, Copy, Debug)]
    pub(super) struct Point {
        pub x: isize,
        pub y: isize,
        pub z: isize,
    }

    #[derive(Clone, Debug)]
    pub(super) struct Cube {
        min_point: Point,
        max_point: Point,
        subtracted: Vec<Cube>,
        pub on: bool,
    }

    impl Cube {
        pub fn new(min_point: Point, max_point: Point, on: bool) -> Self {
            Cube {
                min_point,
                max_point,
                subtracted: Vec::new(),
                on,
            }
        }

        pub fn sub(&mut self, other: &Cube) {
            if let Some(overlap) = self.get_overlap(other) {
                for s in &mut self.subtracted {
                    s.sub(&overlap);
                }
                self.subtracted.push(overlap);
            }
        }

        pub fn get_overlap(&self, other: &Cube) -> Option<Cube> {
            if self.intersects(other) {
                Some(Cube::new(
                    Point {
                        x: cmp::max(self.min_point.x, other.min_point.x),
                        y: cmp::max(self.min_point.y, other.min_point.y),
                        z: cmp::max(self.min_point.z, other.min_point.z),
                    },
                    Point {
                        x: cmp::min(self.max_point.x, other.max_point.x),
                        y: cmp::min(self.max_point.y, other.max_point.y),
                        z: cmp::min(self.max_point.z, other.max_point.z),
                    },
                    self.on,
                ))
            } else {
                None
            }
        }

        pub fn intersects(&self, other: &Cube) -> bool {
            other.max_point.x >= self.min_point.x
                && other.min_point.x <= self.max_point.x
                && other.max_point.y >= self.min_point.y
                && other.min_point.y <= self.max_point.y
                && other.max_point.z >= self.min_point.z
                && other.min_point.z <= self.max_point.z
        }

        pub fn volume(&self) -> isize {
            ((self.max_point.x - self.min_point.x).abs() + 1)
                * ((self.max_point.y - self.min_point.y).abs() + 1)
                * ((self.max_point.z - self.min_point.z).abs() + 1)
                - self.subtracted.iter().fold(0, |a, c| a + c.volume())
        }
    }

    impl<'t> From<Captures<'t>> for Cube {
        fn from(captures: Captures) -> Self {
            fn get_range(captures: &Captures, i1: usize, i2: usize) -> (isize, isize) {
                (
                    captures.get(i1).unwrap().as_str().parse::<isize>().unwrap(),
                    captures.get(i2).unwrap().as_str().parse::<isize>().unwrap(),
                )
            }
            let on = captures.get(1).unwrap().as_str() == "on";

            let (x1, x2) = get_range(&captures, 2, 3);
            let (y1, y2) = get_range(&captures, 4, 5);
            let (z1, z2) = get_range(&captures, 6, 7);
            Cube::new(
                Point {
                    x: x1,
                    y: y1,
                    z: z1,
                },
                Point {
                    x: x2,
                    y: y2,
                    z: z2,
                },
                on,
            )
        }
    }
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day22_part1(b: &mut Bencher) {
        let text = read_to_string("res/day22.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day22_part2(b: &mut Bencher) {
        let text = read_to_string("res/day22.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
