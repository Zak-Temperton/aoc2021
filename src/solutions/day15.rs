use std::collections::BinaryHeap;

struct Location {
    x: usize,
    y: usize,
    value: u64,
}

impl Location {
    pub fn new(x: usize, y: usize, value: u64) -> Self {
        Location { x, y, value }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}
impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.value.partial_cmp(&self.value)
    }
}

impl Eq for Location {}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

pub(crate) fn part1(text: &str) {
    let map = text
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as u64).collect())
        .collect::<Vec<Vec<_>>>();
    find_shortest_path(map);
}

pub(crate) fn part2(text: &str) {
    let mut map = text
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as u64).collect())
        .collect::<Vec<Vec<_>>>();
    extend_map(&mut map);
    find_shortest_path(map);
}

fn find_shortest_path(map: Vec<Vec<u64>>) {
    let width = map.len() - 1;
    let height = map[0].len() - 1;
    let mut explored = vec![vec![u64::MAX; height + 1]; width + 1];
    let mut queue = BinaryHeap::new();
    explored[0][0] = 0;
    queue.push(Location::new(0, 0, 0));
    while let Some(location) = queue.pop() {
        let (x, y, value) = (location.x, location.y, location.value);
        if x == width && y == height {
            break;
        }
        if x < width && map[x + 1][y] + value < explored[x + 1][y] {
            explored[x + 1][y] = value + map[x + 1][y];
            queue.push(Location::new(x + 1, y, explored[x + 1][y]));
        }
        if x > 0 && map[x - 1][y] + value < explored[x - 1][y] {
            explored[x - 1][y] = value + map[x - 1][y];
            queue.push(Location::new(x - 1, y, explored[x - 1][y]));
        }
        if y < height && map[x][y + 1] + value < explored[x][y + 1] {
            explored[x][y + 1] = value + map[x][y + 1];
            queue.push(Location::new(x, y + 1, explored[x][y + 1]));
        }
        if y > 0 && map[x][y - 1] + value < explored[x][y - 1] {
            explored[x][y - 1] = value + map[x][y - 1];
            queue.push(Location::new(x, y - 1, explored[x][y - 1]));
        }
    }
    println!("part2: {}", *explored.last().unwrap().last().unwrap());
}

fn extend_map(map: &mut Vec<Vec<u64>>) {
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

fn increment_column(col: &[u64], num: u64) -> Vec<u64> {
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
