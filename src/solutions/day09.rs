pub fn part1(text: &str) {
    let map: Vec<Vec<u8>> = text
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

pub fn part2(text: &str) {
    let mut map: Vec<Vec<bool>> = text
        .lines()
        .map(|line| line.bytes().map(|b| b != b'9').collect()) // if basin return true else false
        .collect();
    let mut basins = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            // find new basin
            if map[y][x] {
                basins.push(size_of_basin(x, y, &mut map));
            }
        }
    }
    basins.sort_unstable_by(|a, b| b.cmp(a));
    println!("part2: {}", basins[..3].iter().product::<usize>());
}

fn size_of_basin(x: usize, y: usize, map: &mut [Vec<bool>]) -> usize {
    let mut size = 1; // counts itself, function will only be called if location is in a basin
    map[y][x] = false; // fill basin to prevent recounting
    if x > 0 && map[y][x - 1] {
        size += size_of_basin(x - 1, y, map);
    }
    if x < map[0].len() - 1 && map[y][x + 1] {
        size += size_of_basin(x + 1, y, map);
    }
    if y > 0 && map[y - 1][x] {
        size += size_of_basin(x, y - 1, map);
    }
    if y < map.len() - 1 && map[y + 1][x] {
        size += size_of_basin(x, y + 1, map);
    }
    size
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day09_part1(b: &mut Bencher) {
        let text = read_to_string("res/day09.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day09_part2(b: &mut Bencher) {
        let text = read_to_string("res/day09.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
