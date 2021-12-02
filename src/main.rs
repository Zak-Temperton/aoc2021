mod day01;
mod day02;
fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("please choose a day to run");
        println!("cargo run <day>");
        return;
    }
    args.next(); //executable
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
        _ => println!("Choose a valid day"),
    }
}
