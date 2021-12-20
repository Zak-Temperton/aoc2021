use regex::Regex;

pub fn part1(text: &str) {
    // target area: x=137..171, y=-98..-73
    let r = Regex::new(r"target area: x=(?:([\d]+))..(?:([\d]+)), y=-(?:([\d]+))..-(?:([\d]+))")
        .unwrap();
    let captures = r.captures(text).unwrap();
    let x_range: (isize, isize) = (
        captures.get(1).unwrap().as_str().parse().unwrap(),
        captures.get(2).unwrap().as_str().parse().unwrap(),
    );
    let y_range: (isize, isize) = (
        captures.get(3).unwrap().as_str().parse().unwrap(),
        captures.get(4).unwrap().as_str().parse().unwrap(),
    );
    let possible_vx = find_posible_turn_ranges(x_range);
    println!("{}", (0..=find_vy(possible_vx, y_range)).sum::<isize>());
}

///finds possible the range of turns for a valid vx
fn find_posible_turn_ranges((x1, x2): (isize, isize)) -> Vec<(isize, isize)> {
    let mut out = Vec::new();
    for vx in 0..=x2 {
        let mut x = 0;
        let mut min_turn = None;
        for (i, dx) in (0..=vx).rev().enumerate() {
            if x >= x1 {
                if x > x2 {
                    if let Some(min) = min_turn {
                        out.push((min, i as isize));
                        break;
                    } else {
                        return out;
                    }
                } else if dx == 0 {
                    out.push((min_turn.unwrap(), isize::MAX));
                    break;
                }
                if min_turn.is_none() {
                    min_turn = Some(i as isize);
                }
            }
            x += dx;
        }
    }
    out
}

/// finds the maximum vy value
fn find_vy(possible_vx: Vec<(isize, isize)>, (y1, y2): (isize, isize)) -> isize {
    for max_vy in (y2..y1).rev() {
        let min_turns = (max_vy - 1) / 2;
        for &(min, max) in possible_vx.iter() {
            if min_turns >= min && min_turns <= max {
                return max_vy;
            }
        }
    }
    0
}

pub fn part2(text1: &str) {
    let r = Regex::new(r"target area: x=(?:([\d]+))..(?:([\d]+)), y=(?:([\-\d]+))..(?:([\-\d]+))")
        .unwrap();
    let captures = r.captures(text1).unwrap();
    let x_range: (isize, isize) = (
        captures.get(1).unwrap().as_str().parse().unwrap(),
        captures.get(2).unwrap().as_str().parse().unwrap(),
    );
    let y_range: (isize, isize) = (
        captures.get(3).unwrap().as_str().parse().unwrap(),
        captures.get(4).unwrap().as_str().parse().unwrap(),
    );
    println!("{}", valid_initial_velocities(x_range, y_range));
}

fn valid_initial_velocities((x1, x2): (isize, isize), (y1, y2): (isize, isize)) -> isize {
    let mut count = 0;
    for vx in 0..=x2 {
        let mut x = 0;
        let (mut min, mut max) = (-1, -1);
        for (i, v) in (0..=vx).rev().enumerate() {
            x += v;
            if min == -1 && x >= x1 && x <= x2 {
                min = i as isize;
            } else if x > x2 {
                max = i as isize;
                break;
            } else if i == 0 {
                max = isize::MAX;
            }
        }
        if min == -1 {
            continue;
        }
        for vy in y1..-y1 {
            let mut y = vy;
            for i in 1..=max {
                if i > min && y <= y2 && y >= y1 {
                    count += 1;
                    break;
                } else if y < y1 {
                    break;
                }
                y += vy - i;
            }
        }
    }
    count
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
        let text1 = read_to_string("res/day17.txt").unwrap();
        b.iter(|| part2(&text1));
    }
}
