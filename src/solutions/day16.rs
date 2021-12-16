pub(crate) fn part1(text: &str) {
    let packet = text
        .chars()
        .map(|c| from_hex(c).iter().copied())
        .flatten()
        .collect::<Vec<u8>>();

    println!("{}", basic_packet_translation(&packet).0);
}

fn basic_packet_translation(packet: &[u8]) -> (usize, usize) {
    let mut version_sum = from_binary(&packet[0..3]);
    let packet_type_id = from_binary(&packet[3..6]);
    if packet_type_id != 4 {
        let i = packet[6];
        let (packet_len, mut index) = if i == 0 {
            (from_binary(&packet[7..22]) + 22, 22)
        } else {
            (from_binary(&packet[7..18]) + 18, 18)
        };
        while index < packet_len {
            let (sum, i) = basic_packet_translation(&packet[index..]);
            index += i;
            version_sum += sum;
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

pub(crate) fn part2(text: &str) {
    todo!()
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
