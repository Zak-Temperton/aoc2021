fn from_binary(slice: &[u8]) -> usize {
    let mut out = 0;
    for &b in slice {
        out <<= 1;
        out |= b as usize;
    }
    out
}

fn from_hex<'a>(c: char) -> &'a [u8] {
    match c {
        '0' => &[0, 0, 0, 0],
        '1' => &[0, 0, 0, 1],
        '2' => &[0, 0, 1, 0],
        '3' => &[0, 0, 1, 1],
        '4' => &[0, 1, 0, 0],
        '5' => &[0, 1, 0, 1],
        '6' => &[0, 1, 1, 0],
        '7' => &[0, 1, 1, 1],
        '8' => &[1, 0, 0, 0],
        '9' => &[1, 0, 0, 1],
        'A' => &[1, 0, 1, 0],
        'B' => &[1, 0, 1, 1],
        'C' => &[1, 1, 0, 0],
        'D' => &[1, 1, 0, 1],
        'E' => &[1, 1, 1, 0],
        'F' => &[1, 1, 1, 1],
        _ => &[],
    }
}

enum LenType {
    Bits(usize),
    Num(usize),
}

fn get_packet_len(packet: &[u8]) -> (LenType, usize) {
    if packet[6] == 0 {
        (LenType::Bits(from_binary(&packet[7..22]) + 22), 22)
    } else {
        (LenType::Num(from_binary(&packet[7..18])), 18)
    }
}

fn sum_versions(packet: &[u8]) -> (usize, usize) {
    let mut version_sum = from_binary(&packet[0..3]);
    let packet_type_id = from_binary(&packet[3..6]);
    if packet_type_id != 4 {
        let (len, mut index) = get_packet_len(packet);
        match len {
            LenType::Bits(end) => {
                while index < end {
                    let (sum, i) = sum_versions(&packet[index..]);
                    index += i;
                    version_sum += sum;
                }
            }
            LenType::Num(end) => {
                for _ in 0..end {
                    let (sum, i) = sum_versions(&packet[index..]);
                    index += i;
                    version_sum += sum;
                }
            }
        }
        (version_sum, index)
    } else {
        let mut index = 6;
        while packet[index] == 1 {
            index += 5;
        }
        index += 5;
        (version_sum, index)
    }
}

pub fn part1(text: &str) {
    let packet = text
        .chars()
        .map(|c| from_hex(c).iter().copied())
        .flatten()
        .collect::<Vec<u8>>();

    println!("part1: {}", sum_versions(&packet).0);
}

fn get_results_from_expression(packet: &[u8]) -> (usize, Vec<usize>) {
    let (len, mut index) = get_packet_len(packet);
    let mut results = Vec::new();
    match len {
        LenType::Bits(end) => {
            while index < end {
                let (i, res) = evaluate_packet(&packet[index..]);
                index += i;
                results.push(res);
            }
        }
        LenType::Num(end) => {
            for _ in 0..end {
                let (i, res) = evaluate_packet(&packet[index..]);
                index += i;
                results.push(res);
            }
        }
    }
    (index, results)
}

fn evaluate_packet(packet: &[u8]) -> (usize, usize) {
    let packet_type_id = from_binary(&packet[3..6]);
    match packet_type_id {
        //sum
        0 => {
            let (index, res) = get_results_from_expression(packet);
            (index, res.iter().sum())
        }
        //product
        1 => {
            let (index, res) = get_results_from_expression(packet);
            (index, res.iter().product())
        }
        //min
        2 => {
            let (index, res) = get_results_from_expression(packet);
            (index, *res.iter().min().unwrap())
        }
        //max
        3 => {
            let (index, res) = get_results_from_expression(packet);
            (index, *res.iter().max().unwrap())
        }
        //literal
        4 => {
            let mut index = 6;
            let mut res = 0;
            while packet[index] == 1 {
                res <<= 4;
                res |= from_binary(&packet[index + 1..index + 5]);
                index += 5;
            }
            res <<= 4;
            res += from_binary(&packet[index + 1..index + 5]);
            (index + 5, res)
        }
        //greater than
        5 => {
            let (index, res) = get_results_from_expression(packet);
            (index, if res[0] > res[1] { 1 } else { 0 })
        }
        //less than
        6 => {
            let (index, res) = get_results_from_expression(packet);
            (index, if res[0] < res[1] { 1 } else { 0 })
        }
        //equal to
        7 => {
            let (index, res) = get_results_from_expression(packet);
            (index, if res[0] == res[1] { 1 } else { 0 })
        }
        _ => panic!(),
    }
}

pub fn part2(text: &str) {
    let packet = text
        .chars()
        .map(|c| from_hex(c).iter().copied())
        .flatten()
        .collect::<Vec<u8>>();

    println!("part2: {}", evaluate_packet(&packet).1);
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day16_part1(b: &mut Bencher) {
        let text = read_to_string("res/day16.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day16_part2(b: &mut Bencher) {
        let text = read_to_string("res/day16.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
