use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("day01/res/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut last = 0;
    let mut count = -1;
    for line in reader.lines() {
        if let Ok(line) = line {
            let curr: i32 = line.parse().unwrap();
            if curr > last {
                count += 1;
            }
            last = curr;
        }
    }
    println!("{}", count);
}

fn part2() {
    let file = File::open("day01/res/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut measurements = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            measurements.push(line.parse::<u32>().unwrap());
        }
    }
    let mut count = 0;
    for i in 3..measurements.len() {
        if measurements[i - 3] < measurements[i] {
            count += 1;
        }
    }

    println!("{}", count);
}
