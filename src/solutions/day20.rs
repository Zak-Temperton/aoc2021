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
                new_image[y + 2][x + 2] = enhancement[index];
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
    let first = prepend_append(lines.next().unwrap());
    let width = first.len();
    let mut image = vec![vec![false; width]; 2];
    image.push(first);
    image.append(&mut lines.map(prepend_append).collect());
    image.append(&mut vec![vec![false; width]; 2]);
    (image, enhancement)
}

fn prepend_append(line: &str) -> Vec<bool> {
    let mut out = vec![false; 2];
    out.append(&mut as_bools(line));
    out.append(&mut vec![false; 2]);
    out
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
