#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cucumber {
    Down(usize),
    Right(usize),
    None,
}

pub(crate) fn part1(text: &str) {
    let mut sea_floor = text
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'v' => Cucumber::Down(0),
                    '>' => Cucumber::Right(0),
                    '.' => Cucumber::None,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = sea_floor.len();
    let width = sea_floor[0].len();
    let mut count = 0;
    let mut moved = false;
    while !moved {
        count += 1;
        for row in sea_floor.iter_mut() {
            for x in 0..width {
                match row[x] {
                    Cucumber::Right(c) if c != count => {
                        if x < width - 1 && row[x + 1] == Cucumber::None {
                            moved = true;
                            row[x + 1] = Cucumber::Right(count);
                            row[x] = Cucumber::None;
                        } else if x == width - 1
                            && row[0] == Cucumber::None
                            && row[1] != Cucumber::Right(count)
                        {
                            moved = true;
                            row[0] = Cucumber::Right(count);
                            row[x] = Cucumber::None;
                        }
                    }
                    _ => {}
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                match sea_floor[y][x] {
                    Cucumber::Down(c) if c != count => {
                        if y < height - 1 && sea_floor[y + 1][x] == Cucumber::None {
                            moved = true;
                            sea_floor[y + 1][x] = Cucumber::Down(count);
                            sea_floor[y][x] = Cucumber::None;
                        } else if y == height - 1
                            && sea_floor[0][x] == Cucumber::None
                            && sea_floor[1][x] != Cucumber::Down(count)
                        {
                            moved = true;
                            sea_floor[0][x] = Cucumber::Down(count);
                            sea_floor[y][x] = Cucumber::None;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    println!("part1: {}", count);
}

pub(crate) fn part2(_: &str) {}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day25_part1(b: &mut Bencher) {
        let text = read_to_string("res/day25.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day25_part2(b: &mut Bencher) {
        let text = read_to_string("res/day25.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
