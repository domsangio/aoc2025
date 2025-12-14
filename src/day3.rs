fn line_iterate(line: &str) -> i64 {
    let mut left = 0;
    let mut right = 0;

    let mut left_pos = 0;
    let mut right_pos = 0;

    for (i, c) in line.chars().enumerate() {
        let dig: i64 = c.to_digit(10).unwrap().into();

        if i == line.len() - 1 {
            if dig > right {
                right = dig;
                right_pos = i;
            }
            break;
        }

        if dig > left {
            left = dig;
            left_pos = i;
            right = 0;
            right_pos = 0;
            // h / hpos should be set on the next iteration?
        } else if dig == left {
            left = dig;
            left_pos = i;
        } else if dig > right {
            right = dig;
            right_pos = i;
        }
    }

    for (i, c) in line.chars().enumerate() {
        if i == left_pos {
            print!("*{}*", c);
        } else if i == right_pos {
            print!("({})", c);
        } else {
            print!("{}", c);
        }
    }
    println!("");
    println!("");
    return left * 10 + right;
}

pub fn part1(input: &str) -> i64 {
    input.lines().fold(0, |sum, line| sum + line_iterate(line))
}

pub fn line_iterate_part2(line: &str, decimals_needed: usize) -> i64 {
    if decimals_needed == 0 {
        return 0;
    }

    let mut largest_digit = 0;
    let mut largest_digit_pos = 0;

    for (i, c) in line.chars().enumerate() {
        if i + decimals_needed > line.len() {
            break;
        }
        let curr_dig: i64 = c.to_digit(10).unwrap().into();
        if curr_dig > largest_digit {
            largest_digit = curr_dig;
            largest_digit_pos = i;
        }
    }

    // note if a tie, take left gives more room for next digits
    return largest_digit * 10_i64.pow(decimals_needed as u32 - 1)
        + line_iterate_part2(&line[largest_digit_pos + 1..], decimals_needed - 1);
}

pub fn part2(input: &str) -> i64 {
    input.lines().map(|line| line_iterate_part2(line, 12)).sum()
}
