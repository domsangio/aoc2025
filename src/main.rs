mod day1;
mod day2;
mod day3;
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
        _ => eprintln!("Unknown day: {}", day),
    }
}
