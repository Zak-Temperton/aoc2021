mod day01;
mod day02;
mod day03;
fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("please choose a day to run");
        println!("cargo run <day>");
        return;
    }
    args.next(); //executable path
    match args.next().unwrap().as_str() {
        "day1" | "day01" | "1" => {
            println!("day01:");
            day01::part1();
            day01::part2();
        }
        "day2" | "day02" | "2" => {
            println!("day02:");
            day02::part1();
            day02::part2();
        }
        "day3" | "day03" | "3" => {
            println!("day03:");
            day03::part1();
            day03::part2();
        }
        _ => println!("Choose a valid day"),
    }
}
