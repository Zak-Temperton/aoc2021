use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn part1() {
    let file = File::open("res/day06.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut trumpetfish = VecDeque::from([0; 9]);
    line.split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|f| trumpetfish[f] += 1);
    for _ in 0..80 {
        let new_fish = trumpetfish.pop_front().unwrap();
        trumpetfish[6] += new_fish;
        trumpetfish.push_back(new_fish);
    }
    println!("part1: {}", trumpetfish.iter().sum::<usize>());
}

pub(crate) fn part2() {
    let file = File::open("res/day06.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut trumpetfish = VecDeque::from([0; 9]);
    line.split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|f| trumpetfish[f] += 1);
    for _ in 0..256 {
        let new_fish = trumpetfish.pop_front().unwrap();
        trumpetfish[6] += new_fish;
        trumpetfish.push_back(new_fish);
    }
    println!("part2: {}", trumpetfish.iter().sum::<usize>());
}
