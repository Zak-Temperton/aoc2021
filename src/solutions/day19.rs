use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    hash::Hash,
    ops::{Add, Sub},
    str::FromStr,
};

use regex::Regex;

pub fn part1_and_part2(text: &str) {
    let mut beakons = Vec::new();

    let r = Regex::new(r"--- scanner [\d]+ ---").unwrap();
    for line in text.lines() {
        if !line.is_empty() {
            if r.is_match(line) {
                beakons.push(Vec::new());
                continue;
            }
            beakons
                .last_mut()
                .unwrap()
                .push(Point3::from_str(line).unwrap());
        }
    }
    let beakons = beakons.drain(..).map(permutations_of).collect::<Vec<_>>();
    let (zero, scanners) = solve(beakons);
    let mut max = 0;
    for (i, a) in scanners.iter().enumerate() {
        for (j, b) in scanners.iter().enumerate() {
            if i != j {
                let dist = a.dist(b);
                if dist > max {
                    max = dist;
                }
            }
        }
    }

    println!("part1: {}", zero.len());

    println!("part2: {}", max);
}

fn solve(beakons: Vec<Vec<Vec<Point3>>>) -> (HashSet<Point3>, Vec<Point3>) {
    let mut zero = beakons[0][0].iter().cloned().collect::<HashSet<Point3>>();
    let mut unmatched = (1..beakons.len()).collect::<HashSet<usize>>();
    let mut scanner = vec![Point3 { x: 0, y: 0, z: 0 }];

    while !unmatched.is_empty() {
        for &index in unmatched.iter() {
            if let Some(diff) = match_and_merge(&mut zero, &beakons[index]) {
                unmatched.remove(&index);
                scanner.push(diff);
                break;
            }
        }
    }

    (zero, scanner)
}

fn permutations_of(mut beakon: Vec<Point3>) -> Vec<Vec<Point3>> {
    let mut permutations = Vec::new();
    for _ in 0..2 {
        for _ in 0..4 {
            for _ in 0..4 {
                permutations.push(beakon.clone());
                beakon.iter_mut().for_each(|p| p.rot_z());
            }
            beakon.iter_mut().for_each(|p| p.rot_y());
        }
        beakon.iter_mut().for_each(|p| p.rot_x());
    }
    permutations
}

fn match_and_merge(zero: &mut HashSet<Point3>, beakon: &Vec<Vec<Point3>>) -> Option<Point3> {
    let mut differences = HashMap::with_capacity(zero.len() * zero.len());
    for permutation in beakon {
        for p1 in zero.iter() {
            for p2 in permutation.iter() {
                let diff = p2.diff(p1);
                let count = differences.entry(diff).or_insert(0_u8);
                *count += 1;
                if *count >= 12 {
                    merge(zero, permutation, diff);
                    return Some(diff);
                }
            }
        }
        differences.clear();
    }
    None
}

fn merge(zero: &mut HashSet<Point3>, beakon: &Vec<Point3>, diff: Point3) {
    for &p in beakon {
        zero.insert(p - diff);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    pub fn rot_x(&mut self) {
        let tmp = self.y;
        self.y = -self.z;
        self.z = tmp;
    }

    pub fn rot_y(&mut self) {
        let tmp = self.x;
        self.x = -self.z;
        self.z = tmp;
    }

    pub fn rot_z(&mut self) {
        let tmp = self.y;
        self.y = -self.x;
        self.x = tmp;
    }

    pub fn diff(&self, rhs: &Self) -> Self {
        *self - *rhs
    }

    pub fn dist(&self, rhs: &Self) -> i32 {
        let diff = self.diff(rhs);
        diff.x.abs() + diff.y.abs() + diff.z.abs()
    }
}

impl Sub<Point3> for Point3 {
    type Output = Point3;

    fn sub(mut self, rhs: Point3) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl Add<Point3> for Point3 {
    type Output = Point3;

    fn add(mut self, rhs: Point3) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl FromStr for Point3 {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Point3, Infallible> {
        let mut s = s.split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        let z = s.next().unwrap().parse().unwrap();
        Ok(Point3 { x, y, z })
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Point3 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.x ^ self.y ^ self.z);
    }
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day19_part1_and_part2(b: &mut Bencher) {
        let text = read_to_string("res/day19.txt").unwrap();
        b.iter(|| part1_and_part2(&text));
    }
}
