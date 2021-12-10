use std::{collections::VecDeque, fs::read_to_string};

pub(crate) fn part1() {
    let line = read_to_string("res/day06.txt").unwrap();
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
    let line = read_to_string("res/day06.txt").unwrap();
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

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn day06_part1(b: &mut Bencher) {
        b.iter(part1);
    }
    #[bench]
    fn day06_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
