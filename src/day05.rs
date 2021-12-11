use std::fs::read_to_string;

pub(crate) fn part1() {
    let mut map = vec![vec![0; 1000]; 1000];
    for line in read_to_string("res/day05.txt").unwrap().lines() {
        let mut split = line
            .split(" -> ")
            .map(|s| s.split(','))
            .flatten()
            .map(|s| s.parse().unwrap());
        let p1: (i32, i32) = (split.next().unwrap(), split.next().unwrap());
        let p2: (i32, i32) = (split.next().unwrap(), split.next().unwrap());
        if p1.0 == p2.0 || p1.1 == p2.1 {
            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;
            let x_sign = dx.signum();
            let y_sign = dy.signum();
            for i in 0..=dx.abs().max(dy.abs()) {
                map[(p1.0 + i * x_sign) as usize][(p1.1 + i * y_sign) as usize] += 1;
            }
        }
    }
    println!(
        "part1: {}",
        map.iter()
            .flatten()
            .fold(0, |i, &x| if x >= 2 { i + 1 } else { i })
    );
}

pub(crate) fn part2() {
    let mut map = vec![vec![0; 1000]; 1000];
    for line in read_to_string("res/day05.txt").unwrap().lines() {
        let mut split = line
            .split(" -> ")
            .map(|s| s.split(','))
            .flatten()
            .map(|s| s.parse().unwrap());
        let p1: (i32, i32) = (split.next().unwrap(), split.next().unwrap());
        let dx = split.next().unwrap() - p1.0;
        let dy = split.next().unwrap() - p1.1;
        let x_sign = dx.signum();
        let y_sign = dy.signum();
        for i in 0..=dx.abs().max(dy.abs()) {
            map[(p1.0 + i * x_sign) as usize][(p1.1 + i * y_sign) as usize] += 1;
        }
    }
    println!(
        "part2: {}",
        map.iter()
            .flatten()
            .fold(0, |i, &x| if x >= 2 { i + 1 } else { i })
    );
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn day05_part1(b: &mut Bencher) {
        b.iter(part1);
    }
    #[bench]
    fn day05_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
