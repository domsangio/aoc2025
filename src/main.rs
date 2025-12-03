mod day1;
mod day2;
use std::env;
use std::fs;

fn main() {
    let day = env::args().nth(1).expect("Usage: cargo run <day>");
    let day_num = &day[..day.len() - 1]; // "1a" -> "1"
    let input = fs::read_to_string(format!("inputs/{}.txt", day_num)).expect("Failed to read input");

    match day.as_str() {
        "1a" => println!("{}", day1::part1(&input)),
        "1b" => println!("{}", day1::part2(&input)),
        "2a" => println!("{}", day2::part1(&input)),
        "2b" => println!("{}", day2::part2(&input)),
        _ => eprintln!("Unknown day: {}", day),
    }
}
