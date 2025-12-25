// we read the input and saw that the longest string is 10 chars, so we can use u16

use std::collections::VecDeque;

#[derive(Debug)]
struct Switch {
    flip: u16, // binary number of the flip
}

impl Switch {
    fn print(&self) {
        print!("[");
        (0..16).for_each(|i| {
            if (self.flip >> i) & 1 == 1 {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!("]");
    }
}

#[derive(Debug)]
struct ParsedInput {
    end_goal: Switch,
    switches: Vec<Switch>,
    voltages: Vec<u16>, // voltage associated with the bit being flipped, same size as end_goal
}

// parses [##...###] into binary 11000111
fn parse_end_goal(raw_end_goal: &str) -> Switch {
    Switch {
        flip: raw_end_goal[1..raw_end_goal.len()]
            .chars()
            .enumerate()
            .map(|(i, c)| 2_u16.pow(i as u32) * (if c == '#' { 1 } else { 0 }))
            .sum(),
    }
}

// parses the (2,3,5,6,8) into a switch of binary, but doesnt set voltage yet
fn parse_switch_block(switch: &str) -> Switch {
    let switch_binary: u16 = switch[1..switch.len() - 1]
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .map(|pow| 2_u16.pow(pow))
        .sum();
    Switch {
        flip: switch_binary as u16,
    }
}

fn parse_input_line(line: &str) -> ParsedInput {
    let sections: Vec<&str> = line.split_whitespace().collect();
    let end_goal = parse_end_goal(sections[0]);
    let switches: Vec<Switch> = sections[1..sections.len() - 1]
        .iter()
        .map(|switch: &&str| parse_switch_block(*switch))
        .collect();
    let voltages = sections
        .last()
        .unwrap()
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u16>().unwrap())
        .collect();

    ParsedInput {
        end_goal,
        switches,
        voltages,
    }
}

/**
 * input: (Current state s, count of steps = k, bit set of switches already flipped = f)
 * set of visited states = V
 *
 * queue of future states = (0 state, 0 steps, flipped switches = 0)
 *
 * while queue is not empty;
 *     (curr state, curr steps, curr flipped switches) = queue.pop
 *     for switches not in flipped switches
 *         if curr state XOR switch i == end goal
 *             return steps + 1
 *         else
 *             queue.push(curr state xor switch, steps + 1, flipped switches + switch i)
 */

fn bfs1(parsed_input: ParsedInput) -> i64 {
    let mut queue = VecDeque::new();
    let empty_switches = vec![false; parsed_input.switches.len()];
    queue.push_back((0, 0, empty_switches.clone()));

    // TODO never implemented visited states
    while let Some((curr_state, curr_steps, mut curr_flipped_switches)) = queue.pop_front() {
        for (i, switch) in parsed_input.switches.iter().enumerate() {
            if !curr_flipped_switches[i] {
                curr_flipped_switches[i] = true;
                let new_state = curr_state ^ switch.flip;
                if new_state == parsed_input.end_goal.flip {
                    return curr_steps + 1;
                }
                queue.push_back((new_state, curr_steps + 1, curr_flipped_switches.clone()));
            }
        }
    }

    panic!("No solution found");
}

/**
 * Thinking out loud, there are only so many combinations. XOR has the commutative property, so
 * when we flip the same switch twice, it should cancel itself out. If thats true, then in the final
 * solution, the switch is either flipped once or not flipped ever -- so to find the solution, its a
 * tree where a switch is included or not; almost another bit mask to do this? or just DP solution
 * DP(current_state, i) ==> DP(current_state XOR switch i, i+1) + DP(current_state, i+1). This gives us runtime
 * 2^n where n is the number of switches -- can we do better? BFS could possibly be better with lots of memory
 * overhead
 *
 * But we need to find the fewest switches flipped for part 1, so BFS does sound like a good idea
 */

pub fn part1(input: &str) -> i64 {
    // println!("{}", input);
    // input
    //     .lines()
    //     .map(|line| parse_input_line(line))
    //     .for_each(|parsed_input| {
    //         print!("End State: ");
    //         parsed_input.end_goal.print();
    //         parsed_input.switches.iter().enumerate().for_each(|(i, switch)| {
    //             print!("Switch {:2}: ", i);
    //             switch.print();
    //         });
    //         println!("Steps: {}", bfs(parsed_input));
    //     });

    input.lines().map(|line| parse_input_line(line)).map(bfs1).sum()
}


/**
 * input: (Current state s of switch increments, count of steps = k)
 *
 * queue of future states = (0 state of switch increments, 0 steps)
 *
 * while queue is not empty;
 *     (curr state, curr steps, curr flipped switches) = queue.pop
 *     for switches not in flipped switches
 *         increment curr state by switch
 *         if curr state == end goal return num steps + 1
 *         else
 *             queue.push new state, steps + 1        
 */

 fn apply_switch_to_state(curr_state: Vec<u16>, switch: Switch) -> Vec<u16> {
    let mut new_state = curr_state.clone();

    (0..16).map(|i| {
        if 1 << i
    })

    new_state
 }

 fn bfs2(parsed_input: ParsedInput) -> i64 {
    let mut queue = VecDeque::new();
    let new_state: Vec<u16> = vec![0; parsed_input.voltages.len()];
    queue.push_back((new_state, 0));

    while let Some((curr_state, curr_steps)) = queue.pop_front() {
        for (i, switch) in parsed_input.switches.iter().enumerate() {

            if !curr_flipped_switches[i] {
                curr_flipped_switches[i] = true;
                let new_state = curr_state ^ switch.flip;
                if new_state == parsed_input.end_goal.flip {
                    return curr_steps + 1;
                }
                queue.push_back((new_state, curr_steps + 1, curr_flipped_switches.clone()));
            }
        }
    }

    panic!("No solution found");
}

pub fn part2(input: &str) -> i64 {
    0
}
