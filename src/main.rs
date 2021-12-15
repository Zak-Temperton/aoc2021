#![feature(test)]
extern crate test;

use std::fs::read_to_string;

use regex::Regex;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("please choose a day to run");
        println!("cargo run <day>");
        return;
    }
    args.next(); //executable path
    let r_day = Regex::new(r"(?:([\d]+))").unwrap();
    let day = args.next().unwrap();
    let captures = r_day.captures(day.as_str()).unwrap();
    match captures.get(1).unwrap().as_str().parse::<u32>().unwrap() {
        1 => {
            println!("day01:");
            let text = read_to_string("res/day01.txt").unwrap();
            day01::part1(&text);
            day01::part2(&text);
        }
        2 => {
            println!("day02:");
            let text = read_to_string("res/day02.txt").unwrap();
            day02::part1(&text);
            day02::part2(&text);
        }
        3 => {
            println!("day03:");
            let text = read_to_string("res/day03.txt").unwrap();
            day03::part1(&text);
            day03::part2(&text);
        }
        4 => {
            println!("day04:");
            let text = read_to_string("res/day04.txt").unwrap();
            day04::part1(&text);
            day04::part2(&text);
        }
        5 => {
            println!("day05:");
            let text = read_to_string("res/day05.txt").unwrap();
            day05::part1(&text);
            day05::part2(&text);
        }
        6 => {
            println!("day06:");
            let text = read_to_string("res/day06.txt").unwrap();
            day06::part1(&text);
            day06::part2(&text);
        }
        7 => {
            println!("day07:");
            let text = read_to_string("res/day07.txt").unwrap();
            day07::part1(&text);
            day07::part2(&text);
        }
        8 => {
            println!("day08:");
            let text = read_to_string("res/day08.txt").unwrap();
            day08::part1(&text);
            day08::part2(&text);
        }
        9 => {
            println!("day09:");
            let text = read_to_string("res/day09.txt").unwrap();
            day09::part1(&text);
            day09::part2(&text);
        }
        10 => {
            println!("day10:");
            let text = read_to_string("res/day10.txt").unwrap();
            day10::part1(&text);
            day10::part2(&text);
        }
        11 => {
            println!("day11:");
            let text = read_to_string("res/day11.txt").unwrap();
            day11::part1(&text);
            day11::part2(&text);
        }
        12 => {
            println!("day12:");
            let text = read_to_string("res/day12.txt").unwrap();
            day12::part1(&text);
            day12::part2(&text);
        }
        13 => {
            println!("day13:");
            let text = read_to_string("res/day13.txt").unwrap();
            day13::part1(&text);
            day13::part2(&text);
        }
        14 => {
            println!("day14:");
            let text = read_to_string("res/day14.txt").unwrap();
            day14::part1(&text);
            day14::part2(&text);
        }
        _ => println!("Choose a valid day"),
    }
}
