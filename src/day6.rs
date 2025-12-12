fn apply_operand(a: i64, b: i64, operand: char) -> i64 {
    match operand {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("Invalid operand: {}", operand),
    }
}

fn init_operand(operand: char) -> i64 {
    match operand {
        '+' => 0,
        '*' => 1,
        _ => panic!("Invalid operand: {}", operand),
    }
}

pub fn part1(input: &str) -> i64 {
    let binding = input.lines().collect::<Vec<&str>>();
    let (operands_line, numeric_lines) = binding.split_last().unwrap();
    let mut numeric_iters = numeric_lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();
    let mut operands = (*operands_line)
        .split_ascii_whitespace()
        .map(|s| s.parse::<char>().unwrap());

    let mut acc = 0;
    while let Some(operand) = operands.next() {
        // iterate over all numeric_iters and on each iter next once
        // needed to collect the numeric_iters because we repeatedly iterate, previously i had
        // it as an iter and it drained after one interation which isnt what we wanted

        acc += numeric_iters
            .iter_mut()
            .fold(init_operand(operand), |acc, num_iter| {
                let num = num_iter.next().unwrap();
                apply_operand(acc, num, operand)
            });
    }

    acc
}

fn find_next_operand_index(curr_index: usize, operands_line: &Vec<char>) -> usize {
    let mut next_index = curr_index + 1;
    while next_index < operands_line.len() {
        if operands_line[next_index] != ' ' {
            return next_index;
        }
        next_index += 1;
    }
    if next_index == operands_line.len() {
        return next_index + 1;
    }
    return next_index;
}

pub fn part2(input: &str) -> i64 {
    let binding = input.lines().collect::<Vec<&str>>();
    let (operands_line, numeric_lines) = binding.split_last().unwrap();

    let rows = numeric_lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut index = 0;

    let chars = operands_line.chars().collect::<Vec<char>>();

    let mut acc = 0;
    while index < chars.len() {
        let next_operand_index = find_next_operand_index(index, &chars);
        let mut curr_nums = Vec::new();
        let curr_operand = chars[index];
        while index < next_operand_index - 1 {
            // last col is all spaces
            let mut curr_num = 0;
            for row in rows.iter() {
                let curr_char = row[index];
                if curr_char != ' ' {
                    curr_num = curr_num * 10 + curr_char.to_digit(10).unwrap() as i64;
                }
                // curr_num = curr_num * 10 + row[index].to_digit(10).unwrap() as i64;
            }
            curr_nums.push(curr_num);
            index += 1;
        }
        index = next_operand_index;
        let curr_sum = curr_nums
            .into_iter()
            .fold(init_operand(curr_operand), |acc, num| {
                apply_operand(acc, num, curr_operand)
            });

        acc += curr_sum;
    }

    acc
}
