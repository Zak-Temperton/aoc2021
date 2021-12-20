const BORDER_WIDTH: usize = 2;

pub fn part1(text: &str) {
    println!(
        "part1: {}",
        enhance(text, 2)
            .drain(..)
            .flatten()
            .fold(0, |a, p| if p { a + 1 } else { a })
    )
}

pub fn part2(text: &str) {
    println!(
        "part2: {}",
        enhance(text, 50)
            .drain(..)
            .flatten()
            .fold(0, |a, p| if p { a + 1 } else { a })
    )
}

fn enhance(text: &str, turns: usize) -> Vec<Vec<bool>> {
    let (mut image, enhancement) = create_image(text);
    for i in 0..turns {
        let mut new_image =
            vec![
                vec![enhancement[if i & 1 == 0 { 0 } else { 0b111111111 }]; image[0].len() + 2];
                image.len() + 2
            ];
        for x in 0..image[0].len() - 2 {
            for y in 0..image.len() - 2 {
                let mut index = 0;
                for i in 0..3 {
                    for j in 0..3 {
                        index <<= 1;
                        if image[y + i][x + j] {
                            index |= 1;
                        }
                    }
                }
                new_image[y + BORDER_WIDTH][x + BORDER_WIDTH] = enhancement[index];
            }
        }
        image = new_image;
    }
    image
}

fn as_bools(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

fn create_image(text: &str) -> (Vec<Vec<bool>>, Vec<bool>) {
    let mut lines = text.lines();
    let enhancement = as_bools(lines.next().unwrap());
    lines.next();
    let centre = lines.map(with_border).collect::<Vec<_>>();
    let top_bottom = vec![vec![false; centre[0].len()]; BORDER_WIDTH];

    (
        [top_bottom.clone(), centre, top_bottom].concat(),
        enhancement,
    )
}

fn with_border(line: &str) -> Vec<bool> {
    const BORDER: [bool; BORDER_WIDTH] = [false; BORDER_WIDTH];
    [BORDER.to_vec(), as_bools(line), BORDER.to_vec()].concat()
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day20_part1(b: &mut Bencher) {
        let text = read_to_string("res/day20.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day20_part2(b: &mut Bencher) {
        let text1 = read_to_string("res/day20.txt").unwrap();
        b.iter(|| part2(&text1));
    }
}
