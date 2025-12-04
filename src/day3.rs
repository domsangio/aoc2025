// // big pair is x1,x2 st x1 < x2
// fn get_two_max_digits(big_pair: (u8, u8), new_num: u8) -> (u8, u8) {
//     if new_num > big_pair.1 {
//         return (big_pair.1, new_num);
//     } else if new_num > big_pair.0 {
//         return (new_num, big_pair.1);
//     } else {
//         return big_pair;
//     }
// }

// pub fn part1(input: &str) -> i64 {
//     let mut sum: i64 = 0;
//     for line in input.lines() {        
//         println!("line: {}", line);
//         let biggest_pair = line.chars().fold((0, 0), |big_pair, c| {
//             get_two_max_digits(big_pair, c.to_digit(10).unwrap() as u8)
//         });
//         println!("biggest_pair: {:?}", biggest_pair);
//         sum += biggest_pair.1 as i64 * 10 + biggest_pair.0 as i64;
//     }
//     sum
// }

fn line_iterate(line: &str) -> i64 {
    let mut H = 0;
    let mut h = 0;

    let mut Hpos = 0;
    let mut hpos = 0;

    for (i, c) in line.chars().enumerate() {
        let dig: i64 = c.to_digit(10).unwrap().into();
        
        if i == line.len() - 1 {
            if dig > h {
                h = dig;
                hpos = i;
            }
            break;
        }
        
        if dig > H {
            H = dig;
            Hpos = i;
            h = 0;
            hpos = 0;
            // h / hpos should be set on the next iteration?
        } else if dig == H {
            h = dig;
            hpos = i;
        } else if dig > h {
            h = dig;
            hpos = i;
        }
    }

    for (i, c) in line.chars().enumerate() {
        if i == Hpos {
            print!("*{}*", c);
        } else if i == hpos {
            print!("({})", c);
        } else {
            print!("{}", c);
        }
    }
    println!("");
    println!("");
    return H * 10 + h;
}

pub fn part1(input: &str) -> i64 {
    input.lines().fold(0, |sum, line| {
        sum + line_iterate(line)
    })
}

pub fn line_iterate_part2(line: &str, decimals_needed: usize) -> i64 {
    if decimals_needed == 0 {
        return 0;
    }

    println!("line: {}, decimals_needed: {}", line, decimals_needed);

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
    return largest_digit * 10_i64.pow(decimals_needed as u32) + line_iterate_part2(&line[largest_digit_pos + 1..], decimals_needed - 1);
}

fn line_print_part2(line: &str, decimals_needed: usize) {
    if decimals_needed == 0 {
        return;
    }

    let mut largest_digit = 0;
    let mut largest_digit_pos = 0;

    for (i, c) in line.chars().enumerate() {
        if i + decimals_needed > line.len() {
            
}



pub fn part2(input: &str) -> i64 {
    input.lines().fold(0, |sum, line| {
        println!("\n\nNEW LINE: {}", line);
        sum + line_iterate_part2(line, 12)
    })
}

