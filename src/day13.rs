use std::fs::read_to_string;

use regex::Regex;

pub(crate) fn part1() {
    let mut width = 1500;
    let mut height = 1000;
    let mut paper = vec![vec![false; width]; height];
    let text = read_to_string("res/day13.txt").unwrap();
    let mut lines = text.lines();
    width = 0;
    height = 0;
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut split = line.split(',');
        let x = split.next().unwrap().parse::<usize>().unwrap() + 1;
        let y = split.next().unwrap().parse::<usize>().unwrap() + 1;
        if width < x {
            width = x;
        }
        if height < y {
            height = y;
        }
        paper[y - 1][x - 1] = true;
    }
    let fold = lines.next().unwrap();
    let r = Regex::new(r"fold along (?:(x|y))=(?:([\d]+))").unwrap();
    let captures = r.captures(fold).unwrap();
    if captures.get(1).unwrap().as_str() == "x" {
        let x = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        for i in x..width {
            for row in paper.iter_mut().take(height) {
                row[x + x - i] |= row[i];
            }
        }
        width = x;
    } else {
        let y = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        for i in y..height {
            for x in 0..width {
                paper[y + y - i][x] |= paper[i][x];
            }
        }
        height = y;
    }
    println!(
        "part1: {}",
        paper[..height]
            .iter()
            .map(|l| l[..width].iter())
            .flatten()
            .fold(0, |a, &p| if p { a + 1 } else { a })
    );
}

pub(crate) fn part2() {
    let mut width = 1500;
    let mut height = 1000;
    let mut paper = vec![vec![false; width]; height];
    let text = read_to_string("res/day13.txt").unwrap();
    let mut lines = text.lines();
    width = 0;
    height = 0;
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut split = line.split(',');
        let x = split.next().unwrap().parse::<usize>().unwrap() + 1;
        let y = split.next().unwrap().parse::<usize>().unwrap() + 1;
        if width < x {
            width = x;
        }
        if height < y {
            height = y;
        }
        paper[y - 1][x - 1] = true;
    }
    for fold in lines {
        let r = Regex::new(r"fold along (?:(x|y))=(?:([\d]+))").unwrap();
        let captures = r.captures(fold).unwrap();
        if captures.get(1).unwrap().as_str() == "x" {
            let x = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            for i in x..width {
                for row in paper.iter_mut().take(height) {
                    row[x + x - i] |= row[i];
                }
            }
            width = x;
        } else {
            let y = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            for i in y..height {
                for x in 0..width {
                    paper[y + y - i][x] |= paper[i][x];
                }
            }
            height = y;
        }
    }
    let out = paper[..height]
        .iter()
        .map(|l| l[..width].iter().map(|&f| if f { 255 } else { 0 }))
        .flatten()
        .collect::<Vec<_>>();
    image::save_buffer_with_format(
        "day13.png",
        &out,
        width as u32,
        height as u32,
        image::ColorType::L8,
        image::ImageFormat::Png,
    )
    .unwrap();
    println!("part2: day13.png");
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn day13_part1(b: &mut Bencher) {
        b.iter(part1);
    }
    #[bench]
    fn day13_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
