pub fn part1(text: &str) {
    let mut crabs: Vec<i32> = text.split(',').flat_map(|s| s.parse()).collect();
    crabs.sort_unstable();
    let median = crabs[crabs.len() / 2];
    println!(
        "part1: {}",
        crabs.iter().fold(0, |a, c| a + (median - c).abs())
    );
}

pub fn part2(text: &str) {
    let crabs: Vec<isize> = text.split(',').flat_map(|s| s.parse()).collect();
    let (max, min) = max_min(&crabs);

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

fn max_min(crabs: &[isize]) -> (isize, isize) {
    crabs
        .iter()
        .fold((isize::MIN, isize::MAX), |(max, min), &c| {
            if c != 0 && c < min {
                (max, c)
            } else if c > max {
                (c, min)
            } else {
                (max, min)
            }
        })
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day07_part1(b: &mut Bencher) {
        let text = read_to_string("res/day07.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day07_part2(b: &mut Bencher) {
        let text = read_to_string("res/day07.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
