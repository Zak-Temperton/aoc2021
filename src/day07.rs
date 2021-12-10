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
        crabs.iter().fold(0, |a, c| a + (median - c).abs())
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

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn day07_part1(b: &mut Bencher) {
        b.iter(part1);
    }
    #[bench]
    fn day07_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
