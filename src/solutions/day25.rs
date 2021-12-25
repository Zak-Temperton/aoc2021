#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cucumber {
    Down,
    Right,
    None,
}

pub(crate) fn part1(text: &str) {
    let mut sea_floor = text
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'v' => Cucumber::Down,
                    '>' => Cucumber::Right,
                    '.' => Cucumber::None,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = sea_floor.len();
    let width = sea_floor[0].len();
    let mut count = 0;

    println!(
        "part1: {}",
        loop {
            let mut moved = false;
            let mut new_sea_floor = vec![vec![Cucumber::None; width]; height];
            for y in 0..height {
                for x in 0..width {
                    match sea_floor[y][x] {
                        Cucumber::Right => {
                            if x < width - 1 && sea_floor[y][x + 1] == Cucumber::None {
                                moved = true;
                                new_sea_floor[y][x + 1] = Cucumber::Right;
                            } else if x == width - 1 && sea_floor[y][0] == Cucumber::None {
                                moved = true;
                                new_sea_floor[y][0] = Cucumber::Right;
                            } else {
                                new_sea_floor[y][x] = Cucumber::Right;
                            }
                        }
                        Cucumber::Down => new_sea_floor[y][x] = Cucumber::Down,
                        Cucumber::None => {}
                    }
                }
            }

            sea_floor = new_sea_floor;
            new_sea_floor = vec![vec![Cucumber::None; width]; height];
            for y in 0..height {
                for x in 0..width {
                    match sea_floor[y][x] {
                        Cucumber::Down => {
                            if y < height - 1 && sea_floor[y + 1][x] == Cucumber::None {
                                moved = true;
                                new_sea_floor[y + 1][x] = Cucumber::Down;
                            } else if y == height - 1 && sea_floor[0][x] == Cucumber::None {
                                moved = true;
                                new_sea_floor[0][x] = Cucumber::Down;
                            } else {
                                new_sea_floor[y][x] = Cucumber::Down;
                            }
                        }
                        Cucumber::Right => new_sea_floor[y][x] = Cucumber::Right,
                        Cucumber::None => {}
                    }
                }
            }
            count += 1;

            if moved {
                sea_floor = new_sea_floor;
            } else {
                break count;
            }
        }
    );
}
pub(crate) fn part2(_: &str) {
    todo!()
}
