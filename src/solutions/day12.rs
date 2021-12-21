use std::collections::{HashMap, HashSet};

pub fn part1(text: &str) {
    let caves = create_caves(text);
    let mut visited: HashSet<&str> = HashSet::new();
    println!("part1: {}", traverse_caves1("start", &caves, &mut visited));
}

fn create_caves(text: &str) -> HashMap<&str, Vec<&str>> {
    let mut caves = HashMap::new();
    for line in text.lines() {
        let mut paths = line.split('-');
        let path1 = paths.next().unwrap();
        let path2 = paths.next().unwrap();
        caves.entry(path1).or_insert_with(Vec::new).push(path2);
        caves.entry(path2).or_insert_with(Vec::new).push(path1);
    }
    caves
}

fn traverse_caves1<'a>(
    location: &'a str,
    caves: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
) -> usize {
    if location == "end" {
        return 1;
    }
    let mut paths = 0;
    if (location.as_bytes()[0] as char).is_lowercase() {
        visited.insert(location);
    }
    for &cave in caves.get(location).unwrap() {
        if !visited.contains(cave) {
            paths += traverse_caves1(cave, caves, visited);
        }
    }
    visited.remove(location);
    paths
}

pub fn part2(text: &str) {
    let caves = create_caves(text);
    let mut visited = HashSet::new();
    visited.insert("start");
    println!("part2: {}", traverse_caves2("start", &caves, &mut visited));
}

fn traverse_caves2<'a>(
    location: &'a str,
    caves: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
) -> usize {
    if location == "end" {
        return 1;
    }
    let mut paths = 0;
    if (location.as_bytes()[0] as char).is_lowercase() {
        visited.insert(location);
    }
    for &cave in caves.get(location).unwrap() {
        if !visited.contains(cave) {
            paths += traverse_caves2(cave, caves, visited);
        } else if cave != "start" {
            paths += traverse_caves1(cave, caves, visited);
            visited.insert(cave); //would have been removed
        }
    }
    visited.remove(location);
    paths
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day12_part1(b: &mut Bencher) {
        let text = read_to_string("res/day12.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day12_part2(b: &mut Bencher) {
        let text = read_to_string("res/day12.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
