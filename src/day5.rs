fn build_ranges_and_return_rest(input: &str) -> (Vec<(i64, i64)>, &str) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let mut all_ranges = ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();
            (start, end)
        })
        .collect::<Vec<(i64, i64)>>();

    all_ranges.sort();

    return (all_ranges, ingredients);
}

pub fn part1(input: &str) -> i64 {
    // binary search the range on the left. the pivot point is the next element. at that point,
    // we can check if the highest end to the left is greater than the ingredient, if that is the case,
    // then it is fresh because we have a start somewhere that is less than us and the end is larger, without
    // knowing what the actual range that the start or end came from

    let (all_ranges, ingredients) = build_ranges_and_return_rest(input);

    let highest_end_at_this_point = all_ranges
        .iter()
        .map(|range| range.1)
        .scan(-1, |max_end, end| {
            *max_end = end.max(*max_end);
            Some(*max_end)
        })
        .collect::<Vec<i64>>();

    ingredients
        .lines()
        .map(|raw_ingredient| {
            let ingredient = raw_ingredient.parse::<i64>().unwrap();
            println!("ingredient: {}", ingredient);
            let partition_point = all_ranges.partition_point(|range| range.0 <= ingredient);
            println!("partition_point: {}", partition_point);

            if partition_point == 0 {
                return 0;
            } else {
                let prior_range_idx = partition_point - 1;
                if ingredient <= highest_end_at_this_point[prior_range_idx] {
                    return 1;
                } else {
                    return 0;
                }
            }
        })
        .sum()
}

fn merge_ranges(all_ranges: &mut Vec<(i64, i64)>) {
    // merge ranges so they dont overlap (part 2) -- could use for part 1 but i liked how i did it
    let mut pos = 0;
    while pos < all_ranges.len() - 1 {
        if all_ranges[pos].1 >= all_ranges[pos + 1].0 {
            all_ranges[pos].1 = all_ranges[pos + 1].1;
            all_ranges.remove(pos + 1);
        } else {
            pos += 1;
        }
    }
    all_ranges.insert(0, (0, 0))
}

pub fn part2(input: &str) -> i64 {
    let (mut all_ranges, ingredients) = build_ranges_and_return_rest(input);
    merge_ranges(&mut all_ranges);

    let ingredient_ranges = ingredients
        .lines()
        .map(|raw_ingredient| {
            let (start_range, end_range) = raw_ingredient.split_once("-").unwrap();
            (
                start_range.parse::<i64>().unwrap(),
                end_range.parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<(i64, i64)>>();

    let mut fresh_count = 0;
    for (start_ingredient, end_ingredient) in ingredient_ranges {
        let mut curr_range = all_ranges.partition_point(|range| range.0 <= start_ingredient) - 1; // guaranteed because we inserted 0-0
        let mut curr_ingredient_start = start_ingredient;
        let mut curr_ingredient_end = end_ingredient;

        loop {
            if all_ranges[curr_range].1 < curr_ingredient_start {
                curr_range += 1;
            } else {
                let end_of_range = all_ranges[curr_range].1.min(curr_ingredient_end);
                fresh_count += end_of_range - curr_ingredient_start + 1;
                curr_ingredient_start = end_of_range + 1;
                curr_range += 1;
                if curr_range == all_ranges.len() || curr_ingredient_start > curr_ingredient_end {
                    break;
                }
            }
        }
    }

    fresh_count
}
