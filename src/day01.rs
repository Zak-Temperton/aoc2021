use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1() {
    let file = File::open("res/day01.txt").unwrap();
    let reader = BufReader::new(file);
    let mut last = 0;
    let mut count = -1;
    for line in reader.lines().flatten() {
        let curr: i32 = line.parse().unwrap();
        if curr > last {
            count += 1;
        }
        last = curr;
    }
    println!("part1: {}", count);
}

pub fn part2() {
    let file = File::open("res/day01.txt").unwrap();
    let reader = BufReader::new(file);
    let mut measurements = Vec::new();
    for line in reader.lines().flatten() {
        measurements.push(line.parse::<u32>().unwrap());
    }
    let mut count = 0;
    for i in 3..measurements.len() {
        if measurements[i - 3] < measurements[i] {
            count += 1;
        }
    }
    println!("part2: {}", count);
}
