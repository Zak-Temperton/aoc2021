use std::fs::read_to_string;

pub fn part1(text: &String) {
    let mut count = Vec::new();
    let mut len = 0;
    for line in text.lines() {
        if count.is_empty() {
            count = vec![0; line.len()];
        }
        len += 1;
        for (b, c) in line.bytes().zip(count.iter_mut()) {
            if b == b'1' {
                *c += 1;
            }
        }
    }
    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;
    for c in count {
        gamma <<= 1;
        epsilon <<= 1;
        if c >= len / 2 {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }

    println!("part1: {}", gamma * epsilon);
}

pub fn part2(text: &String) {
    let mut bits = 0;
    let mut o2_nums = Vec::new();
    for line in text.lines() {
        if bits == 0 {
            bits = line.len();
        }
        let mut num = 0;
        for b in line.bytes() {
            num <<= 1;
            if b == b'1' {
                num |= 1;
            }
        }
        o2_nums.push(num);
    }
    let mut co2_nums = o2_nums.clone();
    for i in (0..bits).rev() {
        if o2_nums.len() == 1 {
            break;
        } else {
            let o2_count = count_bits(&o2_nums, i);
            if o2_count >= o2_nums.len() - o2_count {
                keep_with_bit(&mut o2_nums, i, 1);
            } else {
                keep_with_bit(&mut o2_nums, i, 0);
            }
        }
    }
    for i in (0..bits).rev() {
        if co2_nums.len() == 1 {
            break;
        } else {
            let co2_count = count_bits(&co2_nums, i);
            if co2_count >= co2_nums.len() - co2_count {
                keep_with_bit(&mut co2_nums, i, 0);
            } else {
                keep_with_bit(&mut co2_nums, i, 1);
            }
        }
    }
    println!("part1: {}", o2_nums[0] * co2_nums[0]);
}

fn count_bits(nums: &[u32], bit: usize) -> usize {
    let mut count = 0;
    for &num in nums.iter() {
        if ((num >> bit) & 1) == 1 {
            count += 1;
        }
    }
    count
}

fn keep_with_bit(nums: &mut Vec<u32>, bit: usize, on_off: u32) {
    let mut new_nums = Vec::new();
    for n in nums.drain(..) {
        if (n >> bit) & 1 == on_off {
            new_nums.push(n);
        }
    }
    *nums = new_nums;
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn day03_part1(b: &mut Bencher) {
        let text = read_to_string("res/day03.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day03_part2(b: &mut Bencher) {
        let text = read_to_string("res/day03.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
