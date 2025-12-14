use std::collections::{HashMap, HashSet};

fn map_lines_to_splitters(input: &str) -> (HashSet<usize>, Vec<HashSet<usize>>) {
    let mut line_iter = input.lines();
    let start_pos: HashSet<usize> = HashSet::from([line_iter
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap()]);

    let line_splitters: Vec<HashSet<usize>> = line_iter
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(i, c)| if c == '^' { (i, true) } else { (i, false) })
                .filter(|(_, is_splitter)| *is_splitter)
                .map(|(i, _)| i)
                .collect::<HashSet<usize>>()
        })
        .collect::<Vec<HashSet<usize>>>();

    (start_pos, line_splitters)
}

pub fn part1(input: &str) -> i64 {
    let (start_pos, line_splitters) = map_lines_to_splitters(input);

    // positions is current position where a beam is and line splitter is the positions of the splitters
    // on current line. so basically, iterate over positions, if curr_line_splitter has it, split it and
    // add to a new set, count increments by the new set size
    line_splitters
        .into_iter()
        .fold((0, start_pos), |(count, positions), curr_line_splitter| {
            let new_positions = positions
                .iter()
                .flat_map(|pos| {
                    if curr_line_splitter.contains(pos) {
                        vec![*pos - 1, *pos + 1]
                    } else {
                        vec![*pos]
                    }
                })
                .collect::<HashSet<usize>>();

            (
                count + positions.intersection(&curr_line_splitter).count(),
                new_positions,
            )
        })
        .0 as i64
}

fn traverse_down(
    beam_pos: usize,
    height: usize,
    line_splitters: &Vec<HashSet<usize>>,
    memoi: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if height == line_splitters.len() - 1 {
        return 1;
    }

    if memoi.contains_key(&(beam_pos, height)) {
        return memoi[&(beam_pos, height)];
    }

    let new_val = if line_splitters[height].contains(&beam_pos) {
        traverse_down(beam_pos - 1, height + 1, line_splitters, memoi)
            + traverse_down(beam_pos + 1, height + 1, line_splitters, memoi)
    } else {
        traverse_down(beam_pos, height + 1, line_splitters, memoi)
    };

    memoi.insert((beam_pos, height), new_val);
    new_val
}

pub fn part2(input: &str) -> i64 {
    let (start_pos, line_splitters) = map_lines_to_splitters(input);
    let mut memoi: HashMap<(usize, usize), i64> = HashMap::new();

    // positions is current position where a beam is and line splitter is the positions of the splitters
    // on current line. so basically, iterate over positions, if curr_line_splitter has it, split it and
    // add to a new set, count increments by the new set size
    traverse_down(
        *start_pos.iter().next().unwrap(),
        1,
        &line_splitters,
        &mut memoi,
    )

    // TODO i think we can just pos + 1 - 1 with old value + new value and skip the recursion
    // because if we split and then collide we just need to increment the num paths at that point
    // where as part 1 is just is there a beam at that point
}
