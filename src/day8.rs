use core::num;
use std::collections::{HashMap, HashSet};

const PART_A_ITERATIONS: usize = 1000;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

struct Circuit {
    points: HashSet<Point>,
}

impl Circuit {
    fn new(a: Point, b: Point) -> Self {
        Self {
            points: HashSet::from([a, b]),
        }
    }
}

fn straight_line_distance(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2)
}

fn get_point_from_line(line: &str) -> Point {
    let nums: Vec<i64> = line
        .split(',')
        .take(3)
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();
    Point::new(nums[0], nums[1], nums[2])
}

fn get_points(input: &str) -> Vec<Point> {
    input.lines().map(get_point_from_line).collect()
}

fn get_pairs_sorted_by_distance(points: &Vec<Point>) -> Vec<(Point, Point, i64)> {
    let mut pairs = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            pairs.push((points[i], points[j], straight_line_distance(&points[i], &points[j])));
        }
    }
    // sort by distance
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    pairs
}

pub fn part1(input: &str) -> i64 {
    let points = get_points(input);
    let pairs = get_pairs_sorted_by_distance(&points);

    // all circuits owns the Circuits, point_to_circuit maps point to index of circuit they are in
    // which will allow us to merge circuits
    let mut all_circuits: Vec<Circuit> = Vec::new();
    let mut point_to_circuit: HashMap<Point, usize> = HashMap::new();

    // TODO what happens when there are equidistant points?

    for pair in pairs.iter().take(PART_A_ITERATIONS) {
        let circuit_a_opt = point_to_circuit.get(&pair.0);
        let circuit_b_opt = point_to_circuit.get(&pair.1);

        match (circuit_a_opt, circuit_b_opt) {
            (None, None) => {
                // neither are in a circuit so we create a new circuit
                let new_circuit = Circuit::new(pair.0, pair.1);
                all_circuits.push(new_circuit);
                let new_circuit_index = all_circuits.len() - 1;
                point_to_circuit.insert(pair.0, new_circuit_index);
                point_to_circuit.insert(pair.1, new_circuit_index);
            }
            (Some(circuit_a_index), Some(circuit_b_index)) => {
                // both are in a circuit, so if they are in a different circuit we must merge
                if circuit_a_index == circuit_b_index {
                    // these are already in the same circuit so do nothing
                    continue;
                }

                let circuit_a_idx = *circuit_a_index;
                let circuit_b_idx = *circuit_b_index;

                // merge circuit B into circuit A
                // first point all points in circuit B to A, then add the points from
                // B into A, then empty circuit B (keep the same because its a vector)
                // if we were to remove the empty circuit it would mess up later indices
                // in the vec

                let circuit_b_points = &all_circuits[circuit_b_idx].points.clone();
                for point in circuit_b_points.iter() {
                    *point_to_circuit.get_mut(point).unwrap() = circuit_a_idx;
                }

                all_circuits[circuit_a_idx].points.extend(circuit_b_points.into_iter());

                all_circuits.get_mut(circuit_b_idx).unwrap().points.clear();
            }
            (Some(circuit_a_index), None) => {
                let circuit_a_idx = *circuit_a_index;
                all_circuits[circuit_a_idx].points.insert(pair.1);
                point_to_circuit.insert(pair.1, circuit_a_idx);
            }
            (None, Some(circuit_b_index)) => {
                let circuit_b_idx = *circuit_b_index;
                all_circuits[circuit_b_idx].points.insert(pair.0);
                point_to_circuit.insert(pair.0, circuit_b_idx);
            }
        }
    }

    let mut smallest_circuits = all_circuits
        .into_iter()
        .filter_map(|circuit| {
            if circuit.points.is_empty() {
                None
            } else {
                Some(circuit.points.len())
            }
        })
        .collect::<Vec<_>>();
    smallest_circuits.sort();

    // rev into largest circuits
    smallest_circuits
        .into_iter()
        .rev()
        .take(3)
        .fold(1, |acc, x| acc * x as i64)
}

pub fn part2(input: &str) -> i64 {
    let points = get_points(input);
    let pairs = get_pairs_sorted_by_distance(&points);

    // all circuits owns the Circuits, point_to_circuit maps point to index of circuit they are in
    // which will allow us to merge circuits
    let mut all_circuits: Vec<Circuit> = Vec::new();
    let mut point_to_circuit: HashMap<Point, usize> = HashMap::new();

    let mut answer = None;
    // TODO what happens when there are equidistant points?
    for pair in pairs.iter() {
        let circuit_a_opt = point_to_circuit.get(&pair.0);
        let circuit_b_opt = point_to_circuit.get(&pair.1);

        match (circuit_a_opt, circuit_b_opt) {
            (None, None) => {
                // neither are in a circuit so we create a new circuit
                let new_circuit = Circuit::new(pair.0, pair.1);
                all_circuits.push(new_circuit);
                let new_circuit_index = all_circuits.len() - 1;
                point_to_circuit.insert(pair.0, new_circuit_index);
                point_to_circuit.insert(pair.1, new_circuit_index);
            }
            (Some(circuit_a_index), Some(circuit_b_index)) => {
                // both are in a circuit, so if they are in a different circuit we must merge
                if circuit_a_index == circuit_b_index {
                    // these are already in the same circuit so do nothing
                    continue;
                }

                let circuit_a_idx = *circuit_a_index;
                let circuit_b_idx = *circuit_b_index;

                // merge circuit B into circuit A
                // first point all points in circuit B to A, then add the points from
                // B into A, then empty circuit B (keep the same because its a vector)
                // if we were to remove the empty circuit it would mess up later indices
                // in the vec

                let circuit_b_points = &all_circuits[circuit_b_idx].points.clone();
                for point in circuit_b_points.iter() {
                    *point_to_circuit.get_mut(point).unwrap() = circuit_a_idx;
                }

                all_circuits[circuit_a_idx].points.extend(circuit_b_points.into_iter());

                all_circuits.get_mut(circuit_b_idx).unwrap().points.clear();
            }
            (Some(circuit_a_index), None) => {
                let circuit_a_idx = *circuit_a_index;
                all_circuits[circuit_a_idx].points.insert(pair.1);
                point_to_circuit.insert(pair.1, circuit_a_idx);
            }
            (None, Some(circuit_b_index)) => {
                let circuit_b_idx = *circuit_b_index;
                all_circuits[circuit_b_idx].points.insert(pair.0);
                point_to_circuit.insert(pair.0, circuit_b_idx);
            }
        }

        let arbitrary_point = point_to_circuit.keys().next().unwrap();
        if all_circuits[*point_to_circuit.get(arbitrary_point).unwrap()]
            .points
            .len()
            == points.len()
        {
            answer = Some(pair.0.x * pair.1.x);
            break;
        }
    }
    match answer {
        Some(answer) => answer,
        None => panic!("No solution found above"),
    }
}
