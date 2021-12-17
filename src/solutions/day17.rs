use std::ops::Range;

use regex::Regex;

pub(crate) fn part1(text: &str) {
    // target area: x=137..171, y=-98..-73
    let r = Regex::new(r"target area: x=(?:([\d]+))..(?:([\d]+)), y=-(?:([\d]+))..-(?:([\d]+))")
        .unwrap();
    let captures = r.captures(text).unwrap();
    let x_range: (usize, usize) = (
        captures.get(1).unwrap().as_str().parse().unwrap(),
        captures.get(2).unwrap().as_str().parse().unwrap(),
    );
    let y_range: (usize, usize) = (
        captures.get(3).unwrap().as_str().parse().unwrap(),
        captures.get(4).unwrap().as_str().parse().unwrap(),
    );
    let possible_vx = find_posible_vx(x_range);
    println!("{}", (0..=find_vy(possible_vx, y_range)).sum::<usize>());
}

///finds possible x_velocities and the range of turns its in bounds
fn find_posible_vx((x1, x2): (usize, usize)) -> Vec<(usize, (usize, usize))> {
    let mut out = Vec::new();
    let mut vx = 0;
    loop {
        let mut x = 0;
        let mut min_turn = None;
        for (i, dx) in (0..=vx).rev().enumerate() {
            if x >= x1 {
                if x > x2 {
                    if min_turn.is_none() {
                        return out;
                    } else {
                        out.push((vx, (min_turn.unwrap(), i)));
                        break;
                    }
                } else if dx == 0 {
                    out.push((vx, (min_turn.unwrap(), usize::MAX)));
                    break;
                }
                if min_turn.is_none() {
                    min_turn = Some(i);
                }
            }
            x += dx;
        }
        vx += 1;
    }
}

/// finds the maximum vy value
fn find_vy(possible_vx: Vec<(usize, (usize, usize))>, (y1, y2): (usize, usize)) -> usize {
    let mut max_vy = y1 - 1;
    while max_vy >= y2 {
        let min_turns = (max_vy - 1) / 2;
        for (_, turn) in possible_vx.iter() {
            if min_turns >= turn.0 && min_turns <= turn.1 {
                return max_vy;
            }
        }
        max_vy -= 1;
    }
    max_vy
}

pub(crate) fn part2(text: &str) {
    todo!()
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day17_part1(b: &mut Bencher) {
        let text = read_to_string("res/day17.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day17_part2(b: &mut Bencher) {
        let text = read_to_string("res/day17.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
