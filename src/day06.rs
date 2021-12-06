use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn part1() {
    let file = File::open("res/day06.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let fish = line.split(',').map(|s| s.parse::<usize>().unwrap());
    let mut trumpetfish = vec![0; 9];
    fish.for_each(|f| trumpetfish[f] += 1);
    for _ in 0..80 {
        let mut new_fish = vec![0; 9];
        for i in 0..8 {
            new_fish[i] = trumpetfish[i + 1];
        }
        new_fish[8] = trumpetfish[0];
        new_fish[6] += trumpetfish[0];
        trumpetfish = new_fish;
    }
    println!("part1: {}", trumpetfish.iter().sum::<usize>());
}

pub(crate) fn part2() {
    let file = File::open("res/day06.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let fish = line.split(',').map(|s| s.parse::<usize>().unwrap());
    let mut trumpetfish = vec![0; 9];
    fish.for_each(|f| trumpetfish[f] += 1);
    for _ in 0..256 {
        let mut new_fish = vec![0; 9];
        for i in 0..8 {
            new_fish[i] = trumpetfish[i + 1];
        }
        new_fish[8] = trumpetfish[0];
        new_fish[6] += trumpetfish[0];
        trumpetfish = new_fish;
    }
    println!("part2: {}", trumpetfish.iter().sum::<usize>());
}
