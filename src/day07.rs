use std::fs::read_to_string;

pub(crate) fn part1() {
    let mut crabs: Vec<i32> = read_to_string("res/day07.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    crabs.sort_unstable();
    let median = crabs[crabs.len() / 2];
    println!(
        "part1: {}",
        crabs.iter().map(|c| (median - c).abs()).sum::<i32>()
    );
}

pub(crate) fn part2() {
    let crabs: Vec<isize> = read_to_string("res/day07.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let mut min_cost = isize::MAX;
    for i in min..=max {
        let mut cost = 0;
        for &c in crabs.iter() {
            let d = (i - c).abs();
            cost += d * (d + 1) / 2;
        }
        if cost < min_cost {
            min_cost = cost;
        }
    }
    println!("part2: {}", min_cost);
}
