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
                        .filter(|&x| !x.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<i32>>(),
                )
            }
            out
        });
    }
    let result = |mut bingos: Vec<Vec<Vec<i32>>>| -> i32 {
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
                            let sum = bingo.iter().flatten().filter(|&&i| i > 0).sum::<i32>();
                            return sum * num;
                        }
                    }
                }
            }
        }
        0
    };
    println!("part1: {}", result(bingos));
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
                        .split(' ')
                        .filter(|&x| !x.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<i32>>(),
                )
            }
            out
        });
    }
    let result = |mut bingos: Vec<Vec<Vec<i32>>>| -> i32 {
        for num in order {
            let mut new_bingos = Vec::new();
            let len = bingos.len();
            for bingo in bingos.iter_mut() {
                let mut count_col = vec![0; 5];
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
                                return bingos[0].iter().flatten().filter(|&&i| i > 0).sum::<i32>()
                                    * num;
                            }
                            won = true;
                        }
                    }
                }
                if !won {
                    new_bingos.push(bingo.clone());
                }
            }
            bingos = new_bingos;
        }
        0
    };
    println!("part2: {}", result(bingos));
}
