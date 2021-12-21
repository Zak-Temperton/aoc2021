use std::collections::VecDeque;

pub fn part1(text: &str) {
    let trumpetfish = solve(text, 80);
    println!("part1: {}", trumpetfish.iter().sum::<usize>());
}

pub fn part2(text: &str) {
    let trumpetfish = solve(text, 256);
    println!("part2: {}", trumpetfish.iter().sum::<usize>());
}

fn solve(text: &str, turns: usize) -> VecDeque<usize> {
    let mut trumpetfish = VecDeque::from([0; 9]);
    text.split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|f| trumpetfish[f] += 1);
    for _ in 0..turns {
        let new_fish = trumpetfish.pop_front().unwrap();
        trumpetfish[6] += new_fish;
        trumpetfish.push_back(new_fish);
    }
    trumpetfish
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day06_part1(b: &mut Bencher) {
        let text = read_to_string("res/day06.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day06_part2(b: &mut Bencher) {
        let text = read_to_string("res/day06.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
