use std::fs::read_to_string;

pub(crate) fn part1() {
    let text = read_to_string("res/day04.txt").unwrap();
    let mut lines = text.lines();
    let mut bingos = Vec::new();
    let order = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    loop {
        if lines.next().is_none() {
            break;
        }
        bingos.push({
            let mut out = Vec::new();
            for _ in 0..5 {
                out.push(
                    lines
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<i32>>(),
                )
            }
            out
        });
    }
    println!("part1: {}", result1(bingos, order));
}

fn result1(mut bingos: Vec<Vec<Vec<i32>>>, order: Vec<i32>) -> i32 {
    for num in order {
        for bingo in bingos.iter_mut() {
            let mut count_col = vec![0; 5];
            for row in bingo.iter_mut() {
                let mut count_row = 0;
                for (c, col) in row.iter_mut().enumerate() {
                    if *col == num {
                        *col = -1;
                        count_row += 1;
                        count_col[c] += 1;
                    } else if *col < 0 {
                        count_col[c] += 1;
                        count_row += 1;
                    }
                    if count_row == 5 || count_col[c] == 5 {
                        return bingos[0].iter().flatten().fold(
                            0,
                            |a, &i| {
                                if i > 0 {
                                    a + i
                                } else {
                                    a
                                }
                            },
                        ) * num;
                    }
                }
            }
        }
    }
    0
}

pub(crate) fn part2() {
    let text = read_to_string("res/day04.txt").unwrap();
    let mut lines = text.lines();
    let mut bingos = Vec::new();
    let order = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    loop {
        if lines.next().is_none() {
            break;
        }
        bingos.push({
            let mut out = Vec::new();
            for _ in 0..5 {
                out.push(
                    lines
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<i32>>(),
                )
            }
            out
        });
    }
    println!("part2: {}", result2(bingos, order));
}

fn result2(mut bingos: Vec<Vec<Vec<i32>>>, order: Vec<i32>) -> i32 {
    for num in order {
        let mut new_bingos = Vec::new();
        let len = bingos.len();
        for bingo in bingos.iter_mut() {
            let mut count_col = [0; 5];
            let mut won = false;
            for row in bingo.iter_mut() {
                let mut count_row = 0;
                for (c, col) in row.iter_mut().enumerate() {
                    if *col == num {
                        *col = -1;
                        count_row += 1;
                        count_col[c] += 1;
                    } else if *col < 0 {
                        count_col[c] += 1;
                        count_row += 1;
                    }
                    if count_row == 5 || count_col[c] == 5 {
                        if len == 1 {
                            return bingos[0].iter().flatten().fold(0, |a, &i| {
                                if i > 0 {
                                    a + i
                                } else {
                                    a
                                }
                            }) * num;
                        }
                        won = true;
                        break;
                    }
                }
                if won {
                    break;
                }
            }
            if !won {
                new_bingos.push(bingo.clone());
            }
        }
        bingos = new_bingos;
    }
    0
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn day04_part1(b: &mut Bencher) {
        b.iter(part1);
    }
    #[bench]
    fn day04_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
