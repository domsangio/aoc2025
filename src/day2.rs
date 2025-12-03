pub fn part1(input: &str) -> i64 {
    let mut count: i64 = 0;
    for parts in input.lines().next().unwrap().split(",") {
        let (start_str, end_str) = parts.split_once("-").unwrap();
        let mut start = start_str.parse::<i64>().unwrap();
        let mut end = end_str.parse::<i64>().unwrap();


        // if it starts odd, start at even amount of digits at 100000
        let mut start_digits = start_str.len();
        if start_digits % 2 != 0 {
            start_digits += 1;
            start = 10_i64.pow(start_digits as u32);
        }
        // if it ends odd, end at even amount of digits at 99999
        let mut end_digits = end_str.len();
        if end_digits % 2 != 0 {
            end_digits -= 1;
            end = 10_i64.pow(end_digits as u32) - 1;
        }

        // handle the first and last number special because we dont start at 1000 or at 9999
        for num_digits in start_digits..=(end_digits + 1) {
            let mut lower_bound = 10_i64.pow(num_digits as u32 - 1).max(start);
            let upper_bound = 10_i64.pow(num_digits as u32).min(end);

            lower_bound = lower_bound.rem_euclid(10_i64.pow(num_digits as u32 / 2));
            if lower_bound < start {
                lower_bound += 10_i64.pow(num_digits as u32 / 2);
            }

            let mut upper_bound = upper_bound.rem_euclid(10_i64.pow(num_digits as u32 / 2));
            if upper_bound + 10_i64.pow(num_digits as u32 / 2) > end {
                
        }
    }

    count
}

pub fn part2(input: &str) -> i32 {
    0
}