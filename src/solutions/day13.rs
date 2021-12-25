pub fn part1(text: &str) {
    let mut paper = vec![vec![false; 1500]; 1000];
    let mut lines = text.lines();
    let (mut width, mut height) = init_paper(&mut lines, &mut paper);
    fold(lines.next().unwrap(), &mut paper, &mut width, &mut height);

    println!(
        "part1: {}",
        paper[..height]
            .iter()
            .flat_map(|l| l[..width].iter())
            .fold(0, |a, &p| if p { a + 1 } else { a })
    );
}

pub fn part2(text: &str) {
    let mut paper = vec![vec![false; 1500]; 1000];
    let mut lines = text.lines();
    let (mut width, mut height) = init_paper(&mut lines, &mut paper);
    for instruction in lines {
        fold(instruction, &mut paper, &mut width, &mut height);
    }
    let out = paper
        .iter()
        .take(height)
        .flat_map(|l| l.iter().take(width).map(|&f| if f { 255 } else { 0 }))
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

fn init_paper(lines: &mut std::str::Lines, paper: &mut [Vec<bool>]) -> (usize, usize) {
    let (mut width, mut height) = (0, 0);
    for line in lines {
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
    (width, height)
}

fn fold(
    // r: &Regex,
    instruction: &str,
    paper: &mut [Vec<bool>],
    width: &mut usize,
    height: &mut usize,
) {
    // let captures = r.captures(instruction).unwrap();
    if &instruction[11..12] == "x" {
        let x = (&instruction[13..]).parse::<usize>().unwrap();
        for i in x..*width {
            for row in paper.iter_mut().take(*height) {
                row[x + x - i] |= row[i];
            }
        }
        *width = x;
    } else {
        let y = (&instruction[13..]).parse::<usize>().unwrap();
        for i in y..*height {
            for x in 0..*width {
                paper[y + y - i][x] |= paper[i][x];
            }
        }
        *height = y;
    }
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day13_part1(b: &mut Bencher) {
        let text = read_to_string("res/day13.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day13_part2(b: &mut Bencher) {
        let text = read_to_string("res/day13.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
