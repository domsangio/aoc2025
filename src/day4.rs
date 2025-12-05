use std::collections::HashMap;

fn get_adjacent_spots(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut adjacent_spots = Vec::new();
    if x > 0 {
        adjacent_spots.push((x - 1, y));
        if y > 0 {
            adjacent_spots.push((x - 1, y - 1));
        }
        if y < max_y {
            adjacent_spots.push((x - 1, y + 1));
        }
    }
    if x < max_x {
        adjacent_spots.push((x + 1, y));
        if y > 0 {
            adjacent_spots.push((x + 1, y - 1));
        }
        if y < max_y {
            adjacent_spots.push((x + 1, y + 1));
        }
    }
    if y > 0 {
        adjacent_spots.push((x, y - 1));
    }
    if y < max_y {
        adjacent_spots.push((x, y + 1));
    }
    adjacent_spots
}
pub fn part1(input: &str) -> i64 {
    let mut roll_map: HashMap<(usize, usize), u8> = HashMap::new();
    let max_x = input.lines().count() - 1;
    let max_y = input.lines().next().unwrap().len() - 1;

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '@' {
                roll_map.insert((x, y), 0);
            }
        }
    }

    // now we just need to iterate over the rolls, add to adjacents
    // Collect keys first to avoid borrow conflict with get_mut
    let keys: Vec<_> = roll_map.keys().copied().collect();
    for (x, y) in keys {
        for (adj_x, adj_y) in get_adjacent_spots(x, y, max_x, max_y) {
            if let Some(count) = roll_map.get_mut(&(adj_x, adj_y)) {
                *count += 1;
            }
        }
    }

    roll_map.values().filter(|&v| *v < 4).count() as i64
}

// reset values to 0, needed because we remove rolls, then reset to 0, then recalc
fn reset_roll_map_values(roll_map: &mut HashMap<(usize, usize), u8>) {
    roll_map.values_mut().for_each(|v| *v = 0);
}

fn calculate_roll_counts(roll_map: &mut HashMap<(usize, usize), u8>, max_x: usize, max_y: usize) {
    let keys: Vec<_> = roll_map.keys().copied().collect();
    for (x, y) in keys {
        for (adj_x, adj_y) in get_adjacent_spots(x, y, max_x, max_y) {
            if let Some(count) = roll_map.get_mut(&(adj_x, adj_y)) {
                *count += 1;
            }
        }
    }
}

fn get_removable_rolls(roll_map: &HashMap<(usize, usize), u8>) -> Vec<(usize, usize)> {
    roll_map
        .iter()
        .filter(|(_, v)| **v < 4)
        .map(|((x, y), _)| (*x, *y))
        .collect()
}

pub fn part2(input: &str) -> i64 {
    let mut roll_map: HashMap<(usize, usize), u8> = HashMap::new();
    let max_x = input.lines().count() - 1;
    let max_y = input.lines().next().unwrap().len() - 1;

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '@' {
                roll_map.insert((x, y), 0);
            }
        }
    }

    // now we just need to iterate over the rolls, add to adjacents
    // Collect keys first to avoid borrow conflict with get_mut
    let mut total_removed = 0;
    loop {
        calculate_roll_counts(&mut roll_map, max_x, max_y);
        let removable_rolls = get_removable_rolls(&roll_map);
        if removable_rolls.is_empty() {
            break;
        }
        total_removed += removable_rolls.len();
        for (x, y) in removable_rolls {
            roll_map.remove(&(x, y));
        }
        reset_roll_map_values(&mut roll_map);
    }
    total_removed as i64
}
