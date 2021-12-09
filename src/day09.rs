use std::fs::read_to_string;

pub(crate) fn part1() {
    let map: Vec<Vec<u8>> = read_to_string("res/day09.txt")
        .unwrap()
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();
    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !((x > 0 && map[y][x - 1] <= map[y][x])
                || (x < map[0].len() - 1 && map[y][x + 1] <= map[y][x])
                || (y > 0 && map[y - 1][x] <= map[y][x])
                || (y < map.len() - 1 && map[y + 1][x] <= map[y][x]))
            {
                count += (map[y][x] + 1) as usize;
            }
        }
    }
    println!("part1: {}", count);
}

pub(crate) fn part2() {
    let mut map: Vec<Vec<bool>> = read_to_string("res/day09.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| if b == b'9' { true } else { false })
                .collect()
        })
        .collect();
    let mut basins = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !map[y][x] {
                basins.push(size_of_basin(x, y, &mut map));
            }
        }
    }
    basins.sort_unstable_by(|a, b| b.cmp(&a));
    println!("part2: {}", basins[..3].iter().product::<usize>());
}

fn size_of_basin(x: usize, y: usize, map: &mut [Vec<bool>]) -> usize {
    if map[y][x] {
        return 0;
    }
    let mut size = 1;
    map[y][x] = true;
    if x > 0 {
        size += size_of_basin(x - 1, y, map);
    }
    if x < map[0].len() - 1 {
        size += size_of_basin(x + 1, y, map);
    }
    if y > 0 {
        size += size_of_basin(x, y - 1, map);
    }
    if y < map.len() - 1 {
        size += size_of_basin(x, y + 1, map);
    }
    size
}
