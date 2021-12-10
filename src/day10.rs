use std::fs::read_to_string;

pub(crate) fn part1() {
    let mut points = 0;
    let mut expected = Vec::new();
    for line in read_to_string("res/day10.txt").unwrap().lines() {
        for c in line.chars() {
            match c {
                '(' => expected.push(')'),
                '[' => expected.push(']'),
                '{' => expected.push('}'),
                '<' => expected.push('>'),
                c if Some(c) != expected.pop() => {
                    points += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    }
                }
                _ => {}
            }
        }
        expected.clear();
    }
    println!("part1: {}", points);
}

pub(crate) fn part2() {
    let mut scores = Vec::new();
    let mut expected = Vec::new();
    for line in read_to_string("res/day10.txt").unwrap().lines() {
        let mut incomplete = true;
        for c in line.chars() {
            match c {
                '(' => expected.push(')'),
                '[' => expected.push(']'),
                '{' => expected.push('}'),
                '<' => expected.push('>'),
                c if Some(c) != expected.pop() => {
                    incomplete = false;
                    break;
                }
                _ => {}
            }
        }
        if incomplete {
            let mut score: u64 = 0;
            for e in expected.iter().rev() {
                score *= 5;
                score += match e {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                }
            }
            scores.push(score);
        }
        expected.clear();
    }
    scores.sort_unstable();
    println!("part2: {}", scores[scores.len() / 2]);
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn day10_part1(b: &mut Bencher) {
        b.iter(part1);
    }
    #[bench]
    fn day10_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
