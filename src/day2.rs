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

fn get_spans(start_str: &str, end_str: &str) -> Vec<(String, String, i64, i64)> {
    let mut span: Vec<(String, String, i64, i64)> = Vec::new();
    let start_num = start_str.parse::<i64>().unwrap();
    let end_num = end_str.parse::<i64>().unwrap();

    let mut curr_digits = start_str.len();
    let mut curr_num = start_num;
    while curr_digits <= end_str.len() {
        if curr_digits % 2 != 0 {
            // 10 ^ digits - 1 keep in mind
            curr_num = 10_i64.pow(curr_digits as u32);
            curr_digits += 1;
        } else {
            let curr_end_num = end_num.min(10_i64.pow(curr_digits as u32) - 1);
            span.push((
                curr_num.to_string(),
                curr_end_num.to_string(),
                curr_num,
                curr_end_num,
            ));
            curr_digits += 2;
            curr_num = 10_i64.pow(curr_digits as u32);
        }
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

// say we start with a number say

// start 1 - 42450123

// let num = 42450123;
// let num_digits = 8;
// let half_num_digits = num_digits / 2;

// let ten_exponent = 10_i64.pow(half_num_digits as u32);
// let first_half = num / ten_exponent;
// let possible_first_num = first_half * ten_exponent + first_half;
// if num < possible_first_num {
//     num = first_half + 1 * ten_exponent;
// }

// start 2 - 42457234

pub fn part2(input: &str) -> i32 {
    0
}

// pub fn part1(input: &str) -> i64 {
//     let mut count: i64 = 0;
//     for parts in input.lines().next().unwrap().split(",") {
//         let (start_str, end_str) = parts.split_once("-").unwrap();
//         let spans = get_spans(start_str, end_str);

//         for (start_str, end_str, start_num, end_num) in spans {
//             count += get_count_in_range_equal_digits(&start_str, &end_str);
//         }
//     }
//     count
// }

// fn get_spans(start_str: &str, end_str: &str) -> Vec<(String, String, i64, i64)> {
//     let mut span: Vec<(String, String, i64, i64)> = Vec::new();
//     let start_num = start_str.parse::<i64>().unwrap();
//     let end_num = end_str.parse::<i64>().unwrap();

//     let mut curr_digits = start_str.len();
//     let mut curr_num = start_num;
//     while curr_digits <= end_str.len() {
//         if curr_digits % 2 != 0 {
//             // 10 ^ digits - 1 keep in mind
//             curr_num = 10_i64.pow(curr_digits as u32);
//             curr_digits += 1;
//         } else {
//             let curr_end_num = end_num.min(10_i64.pow(curr_digits as u32) - 1);
//             span.push((curr_num.to_string(), curr_end_num.to_string(), curr_num, curr_end_num));
//             curr_digits += 2;
//             curr_num = 10_i64.pow(curr_digits as u32);
//         }
//     }

//     return span;
// }

// fn get_count_in_range_equal_digits(start_str: &str, end_str: &str) -> i64 {
//     if start_str.len() != end_str.len() {
//         panic!("Start and end must have the same number of digits");
//     }

//     let start = start_str.parse::<i64>().unwrap();
//     let end = end_str.parse::<i64>().unwrap();

//     let start_half = get_first_num_in_range(start_str, start);
//     let end_half = get_last_num_in_range(end_str, end);

//     if start_half > end_half {
//         return 0;
//     } else if start_half == end_half {
//         return start_half * 10_i64.pow(start_str.len() as u32 / 2) + start_half;
//     } else {
//         let partial_sum = (end_half - start_half + 1) * (start_half + end_half) / 2;
//         return partial_sum * 10_i64.pow(start_str.len() as u32 / 2) + partial_sum;
//     }
// }

// // technically we only need the half when we do this
// fn get_first_num_in_range(start_str: &str, start: i64) -> i64 {
//     /*
//      * Basically map a number here say d1d2d3d4d5d6d7d8 to a number e1e2e3e40000 such that
//      * e1e2e3e4e1e2e3e4 >= d1d2d3d4d5d6d7d8 and e4 <= d4+1 ie this is the next number in the sequence
//      */
//     let num_digits = start_str.len();
//     let half_num_digits = num_digits / 2;
//     let ten_exponent = 10_i64.pow(half_num_digits as u32);
//     let first_half = start / ten_exponent;
//     let possible_first_num = first_half * ten_exponent + first_half;
//     if start <= possible_first_num {
//         first_half // * ten_exponent + first_half
//     } else {
//         (first_half + 1) // * ten_exponent + (first_half + 1)
//     }
// }

// fn get_last_num_in_range(end_str: &str, end: i64) -> i64 {
//     let num_digits = end_str.len();
//     let half_num_digits = num_digits / 2;
//     let ten_exponent = 10_i64.pow(half_num_digits as u32);
//     let first_half = end / ten_exponent;
//     let possible_last_num = first_half * ten_exponent + first_half;

//     if end >= possible_last_num {
//         first_half // * ten_exponent + (first_half - 1)
//     } else {
//         first_half - 1 // * ten_exponent + first_half
//     }
// }

// // say we start with a number say

// // start 1 - 42450123

// // let num = 42450123;
// // let num_digits = 8;
// // let half_num_digits = num_digits / 2;

// // let ten_exponent = 10_i64.pow(half_num_digits as u32);
// // let first_half = num / ten_exponent;
// // let possible_first_num = first_half * ten_exponent + first_half;
// // if num < possible_first_num {
// //     num = first_half + 1 * ten_exponent;
// // }

// // start 2 - 42457234

// pub fn part2(input: &str) -> i32 {
//     0
// }
