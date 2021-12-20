pub fn part1(text: &str) {
    let mut lines = text.lines();
    let enhancement = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();
    lines.next();
    let mut image = create_image(lines);
    for i in 0..2 {
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

    println!(
        "part1: {}",
        image
            .drain(..)
            .flatten()
            .fold(0, |a, p| if p { a + 1 } else { a })
    )
}

pub fn part2(text: &str) {
    let mut lines = text.lines();
    let enhancement = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();
    lines.next();
    let mut image = create_image(lines);
    for i in 0..50 {
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

    println!(
        "part2: {}",
        image
            .drain(..)
            .flatten()
            .fold(0, |a, p| if p { a + 1 } else { a })
    )
}

fn create_image(mut lines: std::str::Lines) -> Vec<Vec<bool>> {
    let mut image = Vec::new();
    let mut first = vec![false; 3];
    first.append(&mut lines.next().unwrap().chars().map(|c| c == '#').collect());
    first.append(&mut vec![false; 3]);
    let width = first.len();
    for _ in 0..3 {
        image.push(vec![false; width]);
    }
    image.push(first);
    image.append(
        &mut lines
            .map(|line| {
                let mut v = vec![false; 3];
                v.append(&mut line.chars().map(|c| c == '#').collect());
                v.append(&mut vec![false; 3]);
                v
            })
            .collect(),
    );
    for _ in 0..3 {
        image.push(vec![false; width]);
    }
    image
}
