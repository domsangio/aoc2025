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
 * num_bits so we dont have to iterate over the entire switch
 */
struct ParsedInput2 {
    num_bits: u8,
    switches: Vec<Switch>,
    voltages: Vec<u16>, // of course the input is larger than u8
}

// parses the (2,3,5,6,8) into a switch of binary, but doesnt set voltage yet
fn parse_switch_block2(switch: &str) -> Switch {
    let switch_binary: u16 = switch[1..switch.len() - 1]
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .map(|pow| 2_u16.pow(pow))
        .sum();
    Switch {
        flip: switch_binary as u16,
    }
}

fn parse_input_line2(line: &str) -> ParsedInput2 {
    let sections: Vec<&str> = line.split_whitespace().collect();
    let switches: Vec<Switch> = sections[1..sections.len() - 1]
        .iter()
        .map(|switch: &&str| parse_switch_block2(*switch))
        .collect();
    let voltages: Vec<u16> = sections
        .last()
        .unwrap()
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u16>().unwrap())
        .collect();

    ParsedInput2 {
        num_bits: voltages.len() as u8,
        switches,
        voltages,
    }
}

/**
 * input: (Current state s of switch increments, count of steps = k, Vec of remaining switches to flip)
 * queue of future states = (0 state of switch increments, 0 steps, remaining switches indices)
 *
 *
 *
 * while queue is not empty;
 *     (curr state, curr steps, curr flipped switches) = queue.pop
 *     for switches not in flipped switches
 *         increment curr state by switch
 *         if curr state == end goal return num steps + 1
 *         else
 *             queue.push new state, steps + 1        
 */

struct State {
    num_bits: u8,
    num_steps: u16,
    curr_counts: Vec<u16>,
    remaining_switches: u16, // bitmask of remaining switches to flip
}

impl State {
    fn initial(target_voltages: &Vec<u16>) -> State {
        State {
            num_bits: target_voltages.len() as u8,
            num_steps: 0,
            curr_counts: vec![0; target_voltages.len()],
            remaining_switches: (1 << target_voltages.len()) - 1,
        }
    }
    // returns None if we overflowed the target voltage, otherwise a new state representing the new state
    // -- is it worth determining remaining switches, and if so when, when constructing a new state, but then we are doing
    // all the time; since we already determine, when we hit None, drop the switch that produced it (outside this function)
    // also outside this function, determine if equal to voltage
    fn apply_switch(&self, switch: &Switch, target_voltage: &Vec<u16>) -> Option<State> {
        let mut new_counts = self.curr_counts.clone();
        for i in 0..self.num_bits {
            if (1 << i) & switch.flip != 0 {
                new_counts[i as usize] += 1;
                if new_counts[i as usize] > target_voltage[i as usize] {
                    return None;
                }
            }
        }

        Some(State {
            num_bits: self.num_bits,
            num_steps: self.num_steps + 1,
            curr_counts: new_counts,
            remaining_switches: self.remaining_switches,
        })
    }

    fn is_target_voltage(&self, target_voltage: &Vec<u16>) -> bool {
        for i in 0..self.num_bits {
            if self.curr_counts[i as usize] != target_voltage[i as usize] {
                return false;
            }
        }
        true
    }
}

// one thing to note is that we did not maintain a master visited, and instead are doing BFS. if there are many steps, we might be in trouble
// here
fn bfs2(parsed_input: ParsedInput2) -> i64 {
    let mut queue = VecDeque::new();
    let starting_state = State::initial(&parsed_input.voltages);
    queue.push_back(starting_state);

    let mut iterations = 0;
    while let Some(current_state) = queue.pop_front() {
        iterations += 1;
        if iterations % 10000 == 0 {
            println!("Iteration: {}", iterations);
        }

        let mut new_states: Vec<State> = Vec::new();
        let mut remaining_switches = current_state.remaining_switches;

        for (i, switch) in parsed_input.switches.iter().enumerate() {
            if (1 << i) & current_state.remaining_switches != 0 {
                match current_state.apply_switch(switch, &parsed_input.voltages) {
                    Some(new_state) => {
                        if new_state.is_target_voltage(&parsed_input.voltages) {
                            return new_state.num_steps as i64;
                        }
                        new_states.push(new_state);
                    }
                    None => {
                        // we deduced that adding switch i will overflow target, which means this is no longer valid
                        // for any of the new states since the new state is monotonic, so we remove it from all next states
                        remaining_switches &= !(1 << i);
                    }
                }
            }
        }

        for new_state in &mut new_states {
            new_state.remaining_switches = remaining_switches;
        }
        queue.extend(new_states);
    }

    panic!("No solution found");
}

pub fn part2(input: &str) -> i64 {
    input.lines().map(|line| parse_input_line2(line)).map(bfs2).sum()
}

#[cfg(test)]
mod tests {
    use crate::day10::{State, Switch};

    #[test]
    fn test_apply_switch() {
        let current_state = State {
            num_bits: 4,
            num_steps: 2,
            curr_counts: vec![1, 2, 3, 4],
            remaining_switches: 0b1,
        };

        let target_voltages: Vec<u16> = vec![4, 4, 4, 4];

        let switch = Switch { flip: 0b1010 };

        let new_state_option = current_state.apply_switch(&switch, &target_voltages);
        assert!(new_state_option.is_some());

        let new_state = new_state_option.unwrap();
        assert_eq!(new_state.num_steps, 3);
        assert_eq!(new_state.curr_counts, vec![2, 2, 4, 4]);
        assert_eq!(new_state.remaining_switches, 0b1);
    }
}
