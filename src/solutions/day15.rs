use std::collections::VecDeque;

pub(crate) fn part1(text: &str) {
    let map = text
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as i64).collect())
        .collect::<Vec<Vec<_>>>();
    let mut explored = vec![vec![i64::MAX; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    explored[0][0] = 0;
    queue.push_back((0, 0));
    while let Some((x, y)) = queue.pop_front() {
        if x < map.len() - 1 && map[x + 1][y] + explored[x][y] < explored[x + 1][y] {
            explored[x + 1][y] = explored[x][y] + map[x + 1][y];
            queue.push_back((x + 1, y));
        }
        if x > 0 && map[x - 1][y] + explored[x][y] < explored[x - 1][y] {
            explored[x - 1][y] = explored[x][y] + map[x - 1][y];
            queue.push_back((x - 1, y));
        }
        if y < map[0].len() - 1 && map[x][y + 1] + explored[x][y] < explored[x][y + 1] {
            explored[x][y + 1] = explored[x][y] + map[x][y + 1];
            queue.push_back((x, y + 1));
        }
        if y > 0 && map[x][y - 1] + explored[x][y] < explored[x][y - 1] {
            explored[x][y - 1] = explored[x][y] + map[x][y - 1];
            queue.push_back((x, y - 1));
        }
    }
    println!("part1: {}", *explored.last().unwrap().last().unwrap());
}

pub(crate) fn part2(text: &str) {
    let mut map = text
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as i64).collect())
        .collect::<Vec<Vec<_>>>();
    extend_map(&mut map);
    let mut explored = vec![vec![i64::MAX; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    explored[0][0] = 0;
    queue.push_back((0, 0));
    while let Some((x, y)) = queue.pop_front() {
        if x < map.len() - 1 && map[x + 1][y] + explored[x][y] < explored[x + 1][y] {
            explored[x + 1][y] = explored[x][y] + map[x + 1][y];
            queue.push_back((x + 1, y));
        }
        if x > 0 && map[x - 1][y] + explored[x][y] < explored[x - 1][y] {
            explored[x - 1][y] = explored[x][y] + map[x - 1][y];
            queue.push_back((x - 1, y));
        }
        if y < map[0].len() - 1 && map[x][y + 1] + explored[x][y] < explored[x][y + 1] {
            explored[x][y + 1] = explored[x][y] + map[x][y + 1];
            queue.push_back((x, y + 1));
        }
        if y > 0 && map[x][y - 1] + explored[x][y] < explored[x][y - 1] {
            explored[x][y - 1] = explored[x][y] + map[x][y - 1];
            queue.push_back((x, y - 1));
        }
    }
    println!("part2: {}", *explored.last().unwrap().last().unwrap());
}

fn extend_map(map: &mut Vec<Vec<i64>>) {
    for col in map.iter_mut() {
        let extention = col.clone();
        for i in 1..5 {
            col.append(&mut increment_column(&extention, i));
        }
    }
    let extention = map.clone();
    for i in 1..5 {
        for col in extention.iter() {
            map.push(increment_column(col, i));
        }
    }
}

fn increment_column(col: &[i64], num: i64) -> Vec<i64> {
    col.iter().map(|i| ((i - 1 + num) % 9) + 1).collect()
}
#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day15_part1(b: &mut Bencher) {
        let text = read_to_string("res/day15.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day15_part2(b: &mut Bencher) {
        let text = read_to_string("res/day15.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
