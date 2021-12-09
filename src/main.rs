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
            day01::part1();
            day01::part2();
        }
        2 => {
            println!("day02:");
            day02::part1();
            day02::part2();
        }
        3 => {
            println!("day03:");
            day03::part1();
            day03::part2();
        }
        4 => {
            println!("day04:");
            day04::part1();
            day04::part2();
        }
        5 => {
            println!("day05:");
            day05::part1();
            day05::part2();
        }
        6 => {
            println!("day06:");
            day06::part1();
            day06::part2();
        }
        7 => {
            println!("day07:");
            day07::part1();
            day07::part2();
        }
        8 => {
            println!("day08:");
            day08::part1();
            day08::part2();
        }
        9 => {
            println!("day09:");
            day09::part1();
            day09::part2();
        }
        _ => println!("Choose a valid day"),
    }
}
