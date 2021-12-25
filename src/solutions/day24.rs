use std::collections::HashMap;

const NUM_LEN: usize = 14;
const SECTION_LEN: usize = 18;
const POW_10: [i64; 14] = [
    10_i64.pow(0),
    10_i64.pow(1),
    10_i64.pow(2),
    10_i64.pow(3),
    10_i64.pow(4),
    10_i64.pow(5),
    10_i64.pow(6),
    10_i64.pow(7),
    10_i64.pow(8),
    10_i64.pow(9),
    10_i64.pow(10),
    10_i64.pow(11),
    10_i64.pow(12),
    10_i64.pow(13),
];

pub fn part1(text: &str) {
    let input = find_valid_modelnum(
        &mut vec![HashMap::new(); NUM_LEN],
        &parse_instructions(text),
        0,
        0,
        &((1..=9).rev().collect::<Vec<_>>()),
    )
    .unwrap();
    println!("part1: {}", input);
}

pub fn part2(text: &str) {
    let input = find_valid_modelnum(
        &mut vec![HashMap::new(); NUM_LEN],
        &parse_instructions(text),
        0,
        0,
        &((1..=9).collect::<Vec<_>>()),
    )
    .unwrap();

    println!("part2: {}", input);
}

#[derive(Clone, Copy)]
enum Register {
    X,
    Y,
    Z,
    W,
    Num(i64),
}

fn get_a_register(line: &str) -> Register {
    match &line[4..5] {
        "x" => Register::X,
        "y" => Register::Y,
        "z" => Register::Z,
        "w" => Register::W,
        _ => panic!(),
    }
}

fn get_b_register(line: &str) -> Register {
    match &line[6..] {
        "x" => Register::X,
        "y" => Register::Y,
        "z" => Register::Z,
        "w" => Register::W,
        b => Register::Num(b.parse().unwrap()),
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Inp(Register),
    Add(Register, Register),
    Mul(Register, Register),
    Div(Register, Register),
    Mod(Register, Register),
    Eql(Register, Register),
}

fn parse_instructions(text: &str) -> Vec<Instruction> {
    let instructions: Vec<Instruction> = text
        .lines()
        .map(|line| -> Instruction {
            match &line[..3] {
                "inp" => Instruction::Inp(get_a_register(line)),
                "add" => Instruction::Add(get_a_register(line), get_b_register(line)),
                "mul" => Instruction::Mul(get_a_register(line), get_b_register(line)),
                "div" => Instruction::Div(get_a_register(line), get_b_register(line)),
                "mod" => Instruction::Mod(get_a_register(line), get_b_register(line)),
                "eql" => Instruction::Eql(get_a_register(line), get_b_register(line)),
                _ => panic!(),
            }
        })
        .collect();
    instructions
}

#[derive(Clone, Copy)]
struct Data {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Data {
    fn get(&self, r: Register) -> i64 {
        match r {
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
            Register::W => self.w,
            Register::Num(n) => n,
        }
    }

    fn get_mut(&mut self, r: Register) -> &mut i64 {
        match r {
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z,
            Register::W => &mut self.w,
            Register::Num(_) => panic!(),
        }
    }
}

fn run_instruction(instruction: Instruction, data: &mut Data) {
    match instruction {
        Instruction::Inp(_) => unreachable!(),
        Instruction::Add(r1, r2) => *data.get_mut(r1) += data.get(r2),
        Instruction::Mul(r1, r2) => *data.get_mut(r1) *= data.get(r2),
        Instruction::Div(r1, r2) => *data.get_mut(r1) /= data.get(r2),
        Instruction::Mod(r1, r2) => *data.get_mut(r1) %= data.get(r2),
        Instruction::Eql(r1, r2) => {
            *data.get_mut(r1) = if data.get(r1) == data.get(r2) { 1 } else { 0 }
        }
    }
}

fn find_valid_modelnum(
    cache: &mut [HashMap<i64, Option<i64>>],
    instructions: &[Instruction],
    index: usize,
    z: i64,
    range: &[i64],
) -> Option<i64> {
    let pc = index * SECTION_LEN + 1;
    if let Some(answer) = cache[index].get(&z) {
        return *answer;
    }
    for &w in range {
        let mut data = Data { x: 0, y: 0, z, w };
        for &instruction in instructions.iter().skip(pc).take(SECTION_LEN - 1) {
            run_instruction(instruction, &mut data);
        }
        if index == NUM_LEN - 1 {
            if data.z == 0 {
                cache[index].insert(data.z, Some(w));
                return Some(w);
            }
        } else if let Some(best) =
            find_valid_modelnum(cache, instructions, index + 1, data.z, range)
        {
            let b = best + w * POW_10[NUM_LEN - 1 - index];
            cache[index].insert(data.z, Some(b));
            return Some(b);
        }
    }

    cache[index].insert(z, None);
    None
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day24_part1(b: &mut Bencher) {
        let text = read_to_string("res/day24.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day24_part2(b: &mut Bencher) {
        let text = read_to_string("res/day24.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
