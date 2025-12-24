// we read the input and saw that the longest string is 10 chars, so we can use u16
struct Switch {
    flip: u16,    // binary number of the flip
    voltage: u16, // voltage associated with the flip
}

struct ParsedInput {
    end_goal: u16,
    switches: Vec<Switch>,
}

// parses [##...###] into binary 11000111
fn parse_end_goal(raw_end_goal: &str) -> u16 {
    raw_end_goal[1..raw_end_goal.len()]
        .chars()
        .enumerate()
        .map(|(i, c)| 2_u16.pow(i as u32) * (if c == '#' { 1 } else { 0 }))
        .sum()
}

// parses the (2,3,5,6,8) into a switch of binary, but doesnt set voltage yet
fn parse_switch_block(switch: &str) -> Switch {
    let switch_binary: u16 = switch[1..switch.len()]
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .map(|pow| 2_u16.pow(pow))
        .sum();
    Switch {
        flip: switch_binary as u16,
        voltage: 0,
    }
}

fn parse_input_line(line: &str) -> ParsedInput {
    let sections: Vec<&str> = line.split_whitespace().collect();
    let end_goal = parse_end_goal(sections[0]);
    let mut switches: Vec<Switch> = sections[1..sections.len() - 1]
        .iter()
        .map(|switch: &&str| parse_switch_block(*switch))
        .collect();
    sections
        .last()
        .unwrap()
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u16>().unwrap())
        .enumerate()
        .for_each(|(i, voltage)| {
            switches[i].voltage = voltage;
        });

    ParsedInput { end_goal, switches }
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(|line| parse_input_line(line)).map(|parsed_input| {});

    return 0;
}

pub fn part2(input: &str) -> i64 {
    0
}
