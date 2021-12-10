use std::fs::read_to_string;

pub(crate) fn part1() {
    let mut count = 0;
    for line in read_to_string("res/day08.txt").unwrap().lines() {
        count += line
            .split('|')
            .skip(1)
            .map(|s| s.split_whitespace())
            .flatten()
            .fold(0, |f, s| match s.len() {
                2 | 3 | 4 | 7 => f + 1,
                _ => f,
            });
    }
    println!("part1: {}", count);
}

pub(crate) fn part2() {
    let mut sum = 0;
    for line in read_to_string("res/day08.txt").unwrap().lines() {
        let mut wires = [0; 7];
        let mut line = line.split('|');
        let mut list = line
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.bytes().map(|b| (b - b'a') as usize).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        list.sort_unstable_by_key(|s| s.len());
        let mut appearances = [0; 7];
        for s in list.iter() {
            for &b in s.iter() {
                appearances[b] += 1;
            }
        }
        for a in 0..7 {
            match appearances[a] {
                4 => wires[a] = 4,
                6 => wires[a] = 1,
                9 => wires[a] = 5,
                _ => {}
            }
        }
        let one = &list[0];
        if wires[one[0]] == 0 {
            wires[one[0]] = 2;
        } else {
            wires[one[1]] = 2;
        }
        let mut top = 0;
        for &i in list[1].iter() {
            if wires[i] == 0 {
                top = i;
                break;
            }
        } // wire already 0
        for &i in list[2].iter() {
            if wires[i] == 0 {
                wires[i] = 3;
                break;
            }
        }
        for &i in list[9].iter() {
            if wires[i] == 0 && i != top {
                wires[i] = 6;
                break;
            }
        }
        let digits = line
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();
        sum += get_output(&wires, &digits);
    }
    println!("part2: {}", sum);
}

fn get_output(wires: &[usize], digits: &[&str]) -> u32 {
    let mut num = 0;
    for digit in digits {
        let mut lights = vec![false; 7];
        for led in digit.bytes() {
            lights[wires[(led - b'a') as usize]] = true;
        }
        num *= 10;
        match lights.as_slice() {
            [true, true, true, false, true, true, true] => num += 0,
            [false, false, true, false, false, true, false] => num += 1,
            [true, false, true, true, true, false, true] => num += 2,
            [true, false, true, true, false, true, true] => num += 3,
            [false, true, true, true, false, true, false] => num += 4,
            [true, true, false, true, false, true, true] => num += 5,
            [true, true, false, true, true, true, true] => num += 6,
            [true, false, true, false, false, true, false] => num += 7,
            [true, true, true, true, true, true, true] => num += 8,
            [true, true, true, true, false, true, true] => num += 9,
            l => {
                println!("fail {:?}", l)
            }
        }
    }
    num
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn day08_part1(b: &mut Bencher) {
        b.iter(part1);
    }
    #[bench]
    fn day08_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
