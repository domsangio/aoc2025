use std::collections::HashSet;

fn get_spans(start_str: &str, end_str: &str) -> Vec<(String, String, i64, i64)> {
    let mut span: Vec<(String, String, i64, i64)> = Vec::new();
    let start_num = start_str.parse::<i64>().unwrap();
    let end_num = end_str.parse::<i64>().unwrap();

    let mut curr_digits = start_str.len();
    let mut curr_num = start_num;
    while curr_digits <= end_str.len() {
        let curr_end_num = end_num.min(10_i64.pow(curr_digits as u32) - 1);
        span.push((curr_num.to_string(), curr_end_num.to_string(), curr_num, curr_end_num));
        curr_digits += 1;
        curr_num = 10_i64.pow(curr_digits as u32 - 1);
    }

    return span;
}

fn get_count_in_range_equal_digits(start_str: &str, end_str: &str) -> i64 {
    if start_str.len() != end_str.len() {
        panic!("Start and end must have the same number of digits");
    }

    let start = start_str.parse::<i64>().unwrap();
    let end = end_str.parse::<i64>().unwrap();

    let start_half = get_first_num_in_range(start_str, start);
    let end_half = get_last_num_in_range(end_str, end);

    if start_half > end_half {
        return 0;
    } else if start_half == end_half {
        return start_half * 10_i64.pow(start_str.len() as u32 / 2) + start_half;
    } else {
        let partial_sum = (end_half - start_half + 1) * (start_half + end_half) / 2;
        return partial_sum * 10_i64.pow(start_str.len() as u32 / 2) + partial_sum;
    }
}

// technically we only need the half when we do this
fn get_first_num_in_range(start_str: &str, start: i64) -> i64 {
    /*
     * Basically map a number here say d1d2d3d4d5d6d7d8 to a number e1e2e3e40000 such that
     * e1e2e3e4e1e2e3e4 >= d1d2d3d4d5d6d7d8 and e4 <= d4+1 ie this is the next number in the sequence
     */

    let num_digits = start_str.len();
    let half_num_digits = num_digits / 2;
    let ten_exponent = 10_i64.pow(half_num_digits as u32);
    let first_half = start / ten_exponent;
    let possible_first_num = first_half * ten_exponent + first_half;
    if start <= possible_first_num {
        first_half // * ten_exponent + first_half
    } else {
        first_half + 1 // * ten_exponent + (first_half + 1)
    }
}

fn get_last_num_in_range(end_str: &str, end: i64) -> i64 {
    let num_digits = end_str.len();
    let half_num_digits = num_digits / 2;
    let ten_exponent = 10_i64.pow(half_num_digits as u32);
    let first_half = end / ten_exponent;
    let possible_last_num = first_half * ten_exponent + first_half;

    if end >= possible_last_num {
        first_half // * ten_exponent + (first_half - 1)
    } else {
        first_half - 1 // * ten_exponent + first_half
    }
}

pub fn part1(input: &str) -> i64 {
    let mut count: i64 = 0;
    for parts in input.lines().next().unwrap().split(",") {
        let (start_str, end_str) = parts.split_once("-").unwrap();
        let spans = get_spans(start_str, end_str);

        for (start_str, end_str, start_num, end_num) in spans {
            count += get_count_in_range_equal_digits(&start_str, &end_str);
        }
    }
    count
}

fn get_repeated_num_from_base_and_digits(base_num: i64, base: usize, num_digits_in_base: usize) -> i64 {
    (0..num_digits_in_base)
        .map(|i| base_num * (10_i64.pow(base as u32)).pow(i as u32))
        .sum()
}

// given a number like 784323401 return 784784784 (the first repeated number)
fn get_first_repeated_num(start_str: &str, start_num: i64, base: usize) -> i64 {
    let num_digits = start_str.len();
    if num_digits % base != 0 {
        panic!("Number of digits must be divisible by base");
    }

    let num_digits_in_base = num_digits / base;
    let base_num = start_str[0..base].parse::<i64>().unwrap();
    let repeated_num = get_repeated_num_from_base_and_digits(base_num, base, num_digits_in_base);
    // do the bounds checking here to make it simpler elsewhere
    if repeated_num >= start_num {
        return base_num;
    } else {
        return base_num + 1;
    }
}

fn get_last_repeated_num(end_str: &str, end_num: i64, base: usize) -> i64 {
    let num_digits = end_str.len();
    if num_digits % base != 0 {
        panic!("Number of digits must be divisible by base");
    }

    let num_digits_in_base = num_digits / base;
    let base_num = end_str[0..base].parse::<i64>().unwrap();
    let repeated_num = get_repeated_num_from_base_and_digits(base_num, base, num_digits_in_base);

    if repeated_num <= end_num {
        return base_num;
    } else {
        return base_num - 1;
    }
}

// doesnt work because we have collisions unfortunately
// fn get_sum_invalid_ids_for_range(
//     start_base: i64,
//     end_base: i64,
//     num_digits: usize,
//     base: usize,
// ) -> i64 {
//     if start_base > end_base {
//         return 0;
//     }

//     // careful of order here due to truncation of integer division
//     let sum_of_nums = (end_base + start_base) * (end_base - start_base + 1) / 2;

//     let repeating_num_length = num_digits / base;
//     let pow_sum = (0..base)
//         .map(|i| 10_i64.pow(repeating_num_length as u32).pow(i as u32))
//         .sum::<i64>();

//     sum_of_nums * pow_sum
// }

fn build_repeated_num(base_num: i64, base: usize, num_digits_in_base: usize) -> i64 {
    (0..num_digits_in_base)
        .map(|i| base_num * (10_i64.pow(base as u32)).pow(i as u32))
        .sum()
}

// given the number is the same base
fn get_repeated_count(start_str: &str, end_str: &str, start_num: i64, end_num: i64) -> i64 {
    // base is the amount of times we repeat the number
    let mut repeated_nums: HashSet<i64> = HashSet::new();

    (1..(start_str.len() / 2 + 1)).for_each(|base| {
        if start_str.len() % base != 0 {
            return;
        }
        let start_base_num = get_first_repeated_num(start_str, start_num, base);
        let end_base_num = get_last_repeated_num(end_str, end_num, base);

        (start_base_num..=end_base_num)
            .map(|base_num| build_repeated_num(base_num, base, start_str.len() / base))
            .for_each(|repeated_num| {
                repeated_nums.insert(repeated_num);
            });
    });

    repeated_nums.iter().sum::<i64>()
}

pub fn part2(input: &str) -> i64 {
    input
        .split(",")
        .map(|start_and_end| {
            let (start_str, end_str) = start_and_end.split_once("-").unwrap();
            let spans = get_spans(start_str, end_str);
            spans
                .iter()
                .map(|(start_str, end_str, start_num, end_num)| {
                    get_repeated_count(start_str, end_str, *start_num, *end_num)
                })
                .sum::<i64>()
        })
        .sum()
}
