use std::collections::HashMap;

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn area(&self, other: &Point) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

pub fn part1(input: &str) -> i64 {
    let points = input
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|(a, b)| Point::new(a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut max_area = 0;
    for point1 in points.iter() {
        for point2 in points.iter() {
            max_area = max_area.max(point1.area(point2));
        }
    }
    max_area
}

#[derive(Clone)]
struct TileSpan {
    start: usize,
    end: usize,
}

#[derive(Clone)]
struct TileLayout {
    rows: HashMap<usize, Vec<TileSpan>>,
    cols: HashMap<usize, Vec<TileSpan>>,
}

impl TileLayout {
    fn new(points: &Vec<Point>) -> Self {
        let x_points = points.iter().map(|p| p.x).collect::<Vec<_>>();
        let y_points = points.iter().map(|p| p.y).collect::<Vec<_>>();

        let x_min = x_points.iter().min().unwrap();
        let x_max = x_points.iter().max().unwrap();
        let y_min = y_points.iter().min().unwrap();
        let y_max = y_points.iter().max().unwrap();

        let mut layout = TileLayout {
            rows: HashMap::new(),
            cols: HashMap::new(),
        };

        for x in ((*(x_min))..=(*(x_max))) {
            layout.rows.insert(x as usize, Vec::new());
        }
        for y in ((*(y_min))..=(*(y_max))) {
            layout.cols.insert(y as usize, Vec::new());
        }
        for window in points.windows(2) {
            if let [p1, p2] = window {
                layout.add_span(p1, p2);
            }
        }
        layout
    }

    fn add_span(&mut self, p1: &Point, p2: &Point) {
        // adding a horizontal span
        if p1.x == p2.x {
            let left_col = p1.y.min(p2.y) as usize;
            let right_col = p1.y.max(p2.y) as usize;

            // ADDING TO ROWS
            let current_row = self.rows.get_mut(&(p1.x as usize)).unwrap();
            // let left_index = current_row.partition_point(|span| left_col < span.start);
            // let right_index = current_row.partition_point(|span| right_col > span.end);
            // // is Binary search even needed here, linear over the spans? is harder for sure

            let mut left_index = 0;
            let mut right_index = current_row.len() - 1;
            while left_index < right_index {
                if current_row[left_index].end + 1 < left_col {
                    left_index += 1;
                } else if right_col < current_row[right_index].start - 1 {
                    right_index -= 1;
                } else {
                    break;
                }
            }

            if left_index == right_index {
                // nothing?
                panic!("Nothing??");
            } else {
                let mut left_spans = current_row[0..left_index].to_vec();
                if left_spans.last().unwrap().end + 1 >= left_col {
                    left_spans.last_mut().unwrap().end = right_col;
                } else {
                    left_spans.push(TileSpan {
                        start: left_col,
                        end: right_col,
                    });
                }

                let right_spans = current_row[right_index + 1..].to_vec();
                if right_spans.first().unwrap().start - 1 <= right_col {
                    left_spans.last_mut().unwrap().end = right_col;
                    left_spans.extend(right_spans.into_iter().skip(1));
                } else {
                    left_spans.extend(right_spans.into_iter());
                }

                *current_row = left_spans;
            }
        }
    }
}

pub fn part2(input: &str) -> i64 {
    let points = input
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|(a, b)| Point::new(a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();

    let layout = TileLayout::new(&points);

    0
}
