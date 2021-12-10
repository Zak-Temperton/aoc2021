use std::fs::read_to_string;

pub fn part1() {
    let mut last = 0;
    let mut count = -1;
    for line in read_to_string("res/day01.txt").unwrap().lines() {
        let curr: i32 = line.parse().unwrap();
        if curr > last {
            count += 1;
        }
        last = curr;
    }
    println!("part1: {}", count);
}

pub fn part2() {
    let measurements = read_to_string("res/day01.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();
    let mut count = 0;
    for i in 3..measurements.len() {
        if measurements[i - 3] < measurements[i] {
            count += 1;
        }
    }
    println!("part2: {}", count);
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn day01_part1(b: &mut Bencher) {
        b.iter(part1);
    }
    #[bench]
    fn day01_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
