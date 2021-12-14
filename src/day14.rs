use std::{collections::HashMap, fs::read_to_string};

pub(crate) fn part1() {
    let text = read_to_string("res/day14.txt").unwrap();
    let mut lines = text.lines();
    let mut template = lines.next().unwrap().as_bytes().to_vec();
    lines.next().unwrap();
    let mut rules = HashMap::new();
    for line in lines {
        let mut split = line.split(" -> ");
        rules.insert(
            split.next().unwrap().as_bytes(),
            split.next().unwrap().as_bytes()[0],
        );
    }

    for _ in 0..10 {
        let mut new_template = Vec::new();
        for i in 0..template.len() - 1 {
            new_template.push(template[i]);
            new_template.push(*rules.get(&template[i..=i + 1]).unwrap());
        }
        new_template.push(*template.last().unwrap());
        template = new_template;
    }
    let mut counts = [0; 26];
    for e in &template {
        counts[(*e - b'A') as usize] += 1;
    }
    println!(
        "part1: {}",
        *counts.iter().max().unwrap()
            - counts
                .iter()
                .fold(i32::MAX, |a, &c| if c != 0 && c < a { c } else { a })
    )
}

pub(crate) fn part2() {
    let text = read_to_string("res/day14.txt").unwrap();
    let mut lines = text.lines();
    let template = lines.next().unwrap().as_bytes().to_vec();
    lines.next().unwrap();
    let mut rules = HashMap::new();
    for line in lines {
        let mut split = line.split(" -> ");
        rules.insert(
            split.next().unwrap().as_bytes(),
            split.next().unwrap().as_bytes()[0],
        );
    }
    let mut rule_result = HashMap::new();
    //loop once to initialise
    for (&k, &r) in rules.iter() {
        let mut count = [0; 26];
        count[(r - b'A') as usize] += 1;
        rule_result.insert(k, count);
    }
    //loops 39 times as we looped once for ititialising
    for _ in 0..39 {
        let mut new_results = HashMap::new();
        for (&k, &r) in rules.iter() {
            let mut count = *rule_result.get(&[k[0], r].as_ref()).unwrap();
            for (i, c) in rule_result
                .get(&[r, k[1]].as_ref())
                .unwrap()
                .iter()
                .enumerate()
            {
                count[i] += c;
            }
            count[(r - b'A') as usize] += 1;
            new_results.insert(k, count);
        }
        rule_result = new_results;
    }
    let mut counts = [0_u64; 26];
    for i in 0..template.len() - 1 {
        counts[(template[i] - b'A') as usize] += 1;
        for (i, c) in rule_result
            .get(&[template[i], template[i + 1]].as_ref())
            .unwrap()
            .iter()
            .enumerate()
        {
            counts[i] += c;
        }
    }
    counts[(*template.last().unwrap() - b'A') as usize] += 1;
    println!(
        "part2: {}",
        *counts.iter().max().unwrap()
            - counts
                .iter()
                .fold(u64::MAX, |a, &c| if c != 0 && c < a { c } else { a })
    )
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn day14_part1(b: &mut Bencher) {
        b.iter(part1);
    }
    #[bench]
    fn day14_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
