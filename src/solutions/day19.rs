use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    hash::Hash,
    ops::{Add, Sub},
    str::FromStr,
};

use regex::Regex;

pub(crate) fn part1(text: &str) {
    let mut scanners = Vec::new();
    let r = Regex::new(r"--- scanner [\d]+ ---").unwrap();
    for line in text.lines() {
        if !line.is_empty() {
            if r.is_match(line) {
                scanners.push(Vec::new());
                continue;
            }
            scanners
                .last_mut()
                .unwrap()
                .push(Point3::from_str(line).unwrap());
        }
    }
    let mut scanners = scanners
        .drain(..)
        .map(|s| permutations_of(s))
        .collect::<Vec<_>>();

    println!("{}", solve(&mut scanners).len());
}

fn solve(scanners: &mut Vec<Vec<Vec<Point3>>>) -> HashSet<Point3> {
    let mut zero = scanners[0][0].iter().cloned().collect::<HashSet<Point3>>();
    let mut unmatched = (1..scanners.len()).collect::<HashSet<usize>>();

    while !unmatched.is_empty() {
        for &index in unmatched.iter() {
            if match_and_merge(&mut zero, scanners[index].clone()) {
                unmatched.remove(&index);
                println!("{}", index);
                break;
            }
        }
    }

    zero
}

fn permutations_of(mut scanner: Vec<Point3>) -> Vec<Vec<Point3>> {
    let mut permutations = Vec::new();
    for _ in 0..2 {
        for _ in 0..4 {
            for _ in 0..4 {
                permutations.push(scanner.clone());
                scanner.iter_mut().for_each(|p| p.rot_z());
            }
            scanner.iter_mut().for_each(|p| p.rot_y());
        }
        scanner.iter_mut().for_each(|p| p.rot_x());
    }
    permutations
}

fn match_and_merge(zero: &mut HashSet<Point3>, scanner: Vec<Vec<Point3>>) -> bool {
    let mut differences = HashMap::with_capacity(zero.len() * zero.len());
    for permutation in scanner {
        for p1 in zero.iter() {
            for p2 in permutation.iter() {
                let diff = p2.diff(p1);
                let count = differences.entry(diff).or_insert(0_u8);
                *count += 1;
                if *count >= 12 {
                    merge(zero, permutation, diff);
                    return true;
                }
            }
        }
        differences.clear();
    }
    false
}

fn merge(zero: &mut HashSet<Point3>, scanner: Vec<Point3>, diff: Point3) {
    for p in scanner {
        zero.insert(p - diff);
    }
}

pub(crate) fn part2(_: &str) {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    pub fn rot_x(&mut self) {
        *self = Self {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }

    pub fn rot_y(&mut self) {
        *self = Self {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    pub fn rot_z(&mut self) {
        *self = Self {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    pub fn diff(&self, rhs: &Self) -> Self {
        *self - *rhs
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

impl Hash for Point3 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.x ^ self.y ^ self.z);
    }
}
