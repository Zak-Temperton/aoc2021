use std::fs::read_to_string;

pub(crate) fn part1(text: &String) {
    let mut octopuses: Vec<Vec<u8>> = text
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    let mut flashes = 0;
    for _ in 0..100 {
        octopuses.iter_mut().flatten().for_each(|o| *o += 1);
        for y in 0..octopuses.len() {
            for x in 0..octopuses[0].len() {
                flashes += try_icrement_charge(x, y, &mut octopuses);
            }
        }
    }
    println!("part1: {}", flashes);
}

pub(crate) fn part2(text: &String) {
    let mut octopuses: Vec<Vec<u8>> = text
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    let mut turn = 0;
    let size = octopuses.len() * octopuses[0].len();
    loop {
        let mut flashes = 0;
        turn += 1;
        octopuses.iter_mut().flatten().for_each(|o| *o += 1);
        for y in 0..octopuses.len() {
            for x in 0..octopuses[0].len() {
                flashes += try_icrement_charge(x, y, &mut octopuses);
            }
        }
        if flashes == size {
            break;
        }
    }
    println!("part2: {}", turn);
}

fn try_icrement_charge(x: usize, y: usize, octopuses: &mut [Vec<u8>]) -> usize {
    if octopuses[y][x] > 9 {
        octopuses[y][x] = 0;
        let mut flashes = 1;
        for i in 0..3 {
            if (x == 0 && i == 0) || (x == octopuses[0].len() - 1 && i == 2) {
                continue;
            }
            for j in 0..3 {
                if (y == 0 && j == 0) || (y == octopuses.len() - 1 && j == 2) || (i == 1 && j == 1)
                {
                    continue;
                }
                let yy = y + j - 1;
                let xx = x + i - 1;
                if octopuses[yy][xx] != 0 {
                    octopuses[yy][xx] += 1;
                    flashes += try_icrement_charge(xx, yy, octopuses);
                }
            }
        }
        flashes
    } else {
        0
    }
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn day11_part1(b: &mut Bencher) {
        let text = read_to_string("res/day11.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day11_part2(b: &mut Bencher) {
        let text = read_to_string("res/day11.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
