
pub fn line_iterator(input: &str, count_iterator: fn(i32, i32) -> i32) -> i32 {
    let mut count = 0;
    let mut curr = 50;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let amount = line[1..].parse::<i32>().unwrap();

        let mut next = curr;
        
        match direction {
            'L' => {
                next -= amount;
            },
            'R' => {
                next += amount;
            },
            _ => {
                panic!("Invalid direction: {}", direction);
            }
        }
        // println!("curr: {}, next: {}, step: {}", curr, next, direction);
        count += count_iterator(curr, next);
        curr = next.rem_euclid(100);
    }

    count
}


pub fn part1(input: &str) -> i32 {
    let count_iterator = |_: i32, next: i32| -> i32 {
        if next % 100 == 0 {
            1
        } else {
            0
        }
    };

    line_iterator(input, count_iterator)
}

pub fn part2(input: &str) -> i32 {
    let count_iterator = |curr: i32, next: i32| -> i32 {
        let ret = 
            if next < 0 {
                if curr == 0 {
                    next.abs() / 100
                } else {
                    next.abs() / 100 + 1
                }
            } else if next >= 100 {
                next / 100
            } else if next == 0 {
                1
            } else {
                0
            };

        // debug
        println!("curr: {}, next: {}, ret: {}", curr, next, ret);
        
        ret
    };

    line_iterator(input, count_iterator)
}