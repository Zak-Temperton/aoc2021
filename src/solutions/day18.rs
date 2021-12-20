//based on https://github.com/paolostyle/advent-of-code-2021/blob/533a5b7fc96786de0e098bf156ee0c9291fb389a/src/bin/day18.rs
use regex::{Captures, Regex};

pub(crate) fn part1(text: &str) {
    let snailfish = text.lines();
    let sum = snailfish
        .map(|s| s.to_string())
        .reduce(add_snailfish_num)
        .unwrap();
    println!("{}", magnitude(sum));
}

fn add_snailfish_num(num1: String, num2: String) -> String {
    let mut snailfish = format!("[{},{}]", num1, num2);
    while reduce(&mut snailfish) {}
    snailfish
}

fn reduce(num: &mut String) -> bool {
    let mut depth = 0;
    let mut replace_begin = 0;
    for (i, c) in num.chars().enumerate() {
        match c {
            '[' => {
                depth += 1;
                if depth > 4 {
                    replace_begin = i;
                }
            }
            ']' => {
                if depth > 4 && replace_begin != 0 {
                    explode(num, replace_begin, i);
                    return true;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    let mut was_num = false;
    for (i, c) in num.chars().enumerate() {
        if c.is_digit(10) {
            if was_num {
                split(num, i - 1);
                return true;
            } else {
                was_num = true;
            }
        } else {
            was_num = false;
        }
    }
    false
}

fn split(num: &mut String, i: usize) {
    let n: u32 = num[i..i + 2].parse().unwrap();
    let left = n / 2;
    let right = n - left;
    let new_pair = format!("[{},{}]", left, right);

    *num = num.replacen(&num[i..i + 2], &new_pair, 1);
}

fn explode(num: &mut String, begin: usize, end: usize) {
    let (left, right): (u32, u32) = {
        let mut iter = num[begin + 1..end].split(',').flat_map(|n| n.parse());
        (iter.next().unwrap(), iter.next().unwrap())
    };
    let mut prefix = num[..begin].to_string();
    let mut suffix = num[end + 1..].to_string();

    let mut digit_index = None;
    for (i, c) in prefix.chars().rev().enumerate() {
        if c.is_digit(10) {
            digit_index = Some(i);
            break;
        }
    }

    if let Some(digit_index) = digit_index {
        let digit_index = prefix.len() - 1 - digit_index;
        let prev_char = prefix.as_bytes()[digit_index - 1] as char;

        if prev_char.is_digit(10) {
            let (l, r) = prefix.split_at(digit_index - 1);
            let new_value = r[..2].parse::<u32>().unwrap() + left;
            prefix = format!("{}{}{}", l, new_value, &r[2..]);
        } else {
            let (l, r) = prefix.split_at(digit_index);
            let new_value = r[..1].parse::<u32>().unwrap() + left;
            prefix = format!("{}{}{}", l, new_value, &r[1..]);
        }
    }

    digit_index = None;
    for (i, c) in suffix.chars().enumerate() {
        if c.is_digit(10) {
            digit_index = Some(i);
            break;
        }
    }

    if let Some(digit_index) = digit_index {
        let next_char = suffix.as_bytes()[digit_index + 1] as char;
        let new_value_str = if next_char.is_digit(10) {
            &suffix[digit_index..digit_index + 2]
        } else {
            &suffix[digit_index..digit_index + 1]
        };
        let new_value = new_value_str.parse::<u32>().unwrap() + right;
        suffix = suffix.replacen(new_value_str, &new_value.to_string(), 1);
    }

    *num = format!("{}0{}", prefix, suffix);
}

fn magnitude(mut num: String) -> u32 {
    let r = Regex::new(r"\[(\d+),(\d+)]").unwrap();

    loop {
        let new_result = r
            .replace_all(&num, |caps: &Captures| {
                let num_1: i32 = caps[1].parse().unwrap();
                let num_2: i32 = caps[2].parse().unwrap();
                format!("{}", 3 * num_1 + 2 * num_2)
            })
            .to_string();

        if new_result == num {
            break num = new_result;
        } else {
            num = new_result;
        }
    }

    num.parse().unwrap()
}

pub(crate) fn part2(text: &str) {
    let lines = text.lines();
    let mut max = 0;
    for (i, num1) in lines.clone().enumerate() {
        for (j, num2) in lines.clone().enumerate() {
            if i != j {
                let mag = magnitude(add_snailfish_num(num1.to_string(), num2.to_string()));
                if mag > max {
                    max = mag;
                }
            }
        }
    }
    println!("{}", max);
}
#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day18_part1(b: &mut Bencher) {
        let text = read_to_string("res/day18.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    #[ignore = "too slow"]
    fn day18_part2(b: &mut Bencher) {
        let text1 = read_to_string("res/day17.txt").unwrap();
        b.iter(|| part2(&text1));
    }
}
