use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn part1() {
    let file = File::open("res/day05.txt").unwrap();
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    for line in reader.lines().flatten() {
        let mut split = line.split(" -> ");
        let mut p1 = split.next().unwrap().split(',');
        let mut p2 = split.next().unwrap().split(',');
        let p1: (i32, i32) = (
            p1.next().unwrap().parse().unwrap(),
            p1.next().unwrap().parse().unwrap(),
        );
        let p2: (i32, i32) = (
            p2.next().unwrap().parse().unwrap(),
            p2.next().unwrap().parse().unwrap(),
        );
        if p1.0 == p2.0 || p1.1 == p2.1 {
            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;
            for i in 0..=dx.abs().max(dy.abs()) {
                *map.entry((p1.0 + i * dx.signum(), p1.1 + i * dy.signum()))
                    .or_insert(0) += 1;
            }
        }
    }
    println!("part1: {}", map.iter().filter(|(_, &x)| x >= 2).count());
}

pub(crate) fn part2() {
    let file = File::open("res/day05.txt").unwrap();
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    for line in reader.lines().flatten() {
        let mut split = line.split(" -> ");
        let mut p1 = split.next().unwrap().split(',');
        let mut p2 = split.next().unwrap().split(',');
        let p1: (i32, i32) = (
            p1.next().unwrap().parse().unwrap(),
            p1.next().unwrap().parse().unwrap(),
        );
        let p2: (i32, i32) = (
            p2.next().unwrap().parse().unwrap(),
            p2.next().unwrap().parse().unwrap(),
        );
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        for i in 0..=dx.abs().max(dy.abs()) {
            *map.entry((p1.0 + i * dx.signum(), p1.1 + i * dy.signum()))
                .or_insert(0) += 1;
        }
    }
    println!("part2: {}", map.iter().filter(|(_, &x)| x >= 2).count());
}
