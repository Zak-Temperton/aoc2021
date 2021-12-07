use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_file(path: &str) -> Vec<usize> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let positions = line.split(',').map(|s| s.parse::<usize>().unwrap());
    let max = positions.clone().max().unwrap();
    let mut crabs = vec![0; max + 1];
    positions.for_each(|c| crabs[c] += 1);
    crabs
}

pub(crate) fn part1() {
    let crabs = parse_file("res/day07.txt");

    let mut left_distances = Vec::with_capacity(crabs.len());
    left_distances.push((0, crabs[0]));
    for l in 1..crabs.len() {
        left_distances.push((
            left_distances[l - 1].0 + left_distances[l - 1].1,
            crabs[l] + left_distances[l - 1].1,
        ));
    }

    let mut right_distances = vec![(0, 0); crabs.len()];
    let right = crabs.len() - 1;
    right_distances[right] = (0, crabs[crabs.len() - 1]);
    for r in (0..right).rev() {
        right_distances[r] = (
            right_distances[r + 1].0 + right_distances[r + 1].1,
            crabs[r] + right_distances[r + 1].1,
        );
    }

    let mut min = left_distances[0].0 + right_distances[0].0;
    for ((l, _), (r, _)) in left_distances.iter().zip(right_distances.iter()) {
        let sum = l + r;
        if sum < min {
            min = sum;
        }
    }

    println!("part1: {}", min);
}

pub(crate) fn part2() {
    let crabs = parse_file("res/day07.txt");

    let mut fuel = vec![0; crabs.len()];
    for i in 0..crabs.len() {
        //right of crab
        let mut f = 1;
        for j in i + 1..crabs.len() {
            fuel[j] += crabs[i] * f;
            f += j - i + 1;
        }
        //left of crab
        f = 1;
        for j in (0..i).rev() {
            fuel[j] += crabs[i] * f;
            f += i - j + 1;
        }
    }
    println!("part2: {}", fuel.iter().min().unwrap());
}
