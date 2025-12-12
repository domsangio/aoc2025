mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
use std::env;
use std::fs;

fn main() {
    let day = env::args().nth(1).expect("Usage: cargo run <day>");

    // Parse: "2at" -> day_num="2", part="a", test=true
    let is_test = day.ends_with('t');
    let day_trimmed = if is_test { &day[..day.len() - 1] } else { &day };
    let day_num = &day_trimmed[..day_trimmed.len() - 1];

    let input_file = if is_test {
        format!("inputs/{}t.txt", day_num)
    } else {
        format!("inputs/{}.txt", day_num)
    };
    let input = fs::read_to_string(&input_file).expect(&format!("Failed to read {}", input_file));

    match day_trimmed {
        "1a" => println!("{}", day1::part1(&input)),
        "1b" => println!("{}", day1::part2(&input)),
        "2a" => println!("{}", day2::part1(&input)),
        "2b" => println!("{}", day2::part2(&input)),
        "3a" => println!("{}", day3::part1(&input)),
        "3b" => println!("{}", day3::part2(&input)),
        "4a" => println!("{}", day4::part1(&input)),
        "4b" => println!("{}", day4::part2(&input)),
        "5a" => println!("{}", day5::part1(&input)),
        "5b" => println!("{}", day5::part2(&input)),
        "6a" => println!("{}", day6::part1(&input)),
        "6b" => println!("{}", day6::part2(&input)),
        "7a" => println!("{}", day7::part1(&input)),
        "7b" => println!("{}", day7::part2(&input)),
        "8a" => println!("{}", day8::part1(&input)),
        "8b" => println!("{}", day8::part2(&input)),
        "9a" => println!("{}", day9::part1(&input)),
        "9b" => println!("{}", day9::part2(&input)),
        "10a" => println!("{}", day10::part1(&input)),
        "10b" => println!("{}", day10::part2(&input)),
        "11a" => println!("{}", day11::part1(&input)),
        "11b" => println!("{}", day11::part2(&input)),
        "12a" => println!("{}", day12::part1(&input)),
        "12b" => println!("{}", day12::part2(&input)),
        _ => eprintln!("Unknown day: {}", day),
    }
}
