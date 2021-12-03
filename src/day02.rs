use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1() {
    let file = File::open("res/day02.txt").unwrap();
    let reader = BufReader::new(file);
    let mut x = 0;
    let mut y = 0;
    for line in reader.lines().flatten() {
        let bytes = line.as_bytes();
        match bytes[0] {
            b'f' => x += (bytes.last().unwrap() - b'0') as u32,
            b'u' => y -= (bytes.last().unwrap() - b'0') as u32,
            b'd' => y += (bytes.last().unwrap() - b'0') as u32,
            _ => panic!(),
        }
    }
    println!("part1: {}", x * y);
}

pub fn part2() {
    let file = File::open("res/day02.txt").unwrap();
    let reader = BufReader::new(file);
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for line in reader.lines().flatten() {
        let bytes = line.as_bytes();
        match bytes[0] {
            b'f' => {
                let f = (bytes.last().unwrap() - b'0') as i32;
                x += f;
                y += aim * f;
            }
            b'u' => aim -= (bytes.last().unwrap() - b'0') as i32,
            b'd' => aim += (bytes.last().unwrap() - b'0') as i32,
            _ => panic!(),
        }
    }
    println!("part2: {}", x * y);
}
