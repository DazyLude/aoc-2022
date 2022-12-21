// #![allow(dead_code, unused_variables)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let sbpairs = get_sensor_beacon_pairs_from_input();
    let sensor_sois = get_sensor_sois(&sbpairs);
    let beacon_set = get_beacon_set(&sbpairs);

    println!(
        "pt1: {}",
        get_covered_in_row_count(&sensor_sois, &beacon_set, 2000000)
    );

    'line_loop: for line_n in 0..=4000000 {
        let mut x = 0;
        let mut check_this = Point::with_coords(x, line_n);
        while let Some(new_x) = is_covered_then_get_next(&check_this, &sensor_sois) {
            if new_x >= 4000000 {
                continue 'line_loop;
            }
            x = new_x;
            check_this.x = new_x;
        }
        println!("pt2: {}", x * 4000000 + line_n);
    }
}

fn is_covered_then_get_next(point: &Point, sensors: &HashMap<Point, i64>) -> Option<i64> {
    for (sensor_coords, soi) in sensors {
        if (*point - *sensor_coords).len() <= *soi {
            return Some(sensor_coords.x + *soi - (point.y - sensor_coords.y).abs() + 1);
        }
    }
    None
}

fn get_covered_in_row_count(
    sensors: &HashMap<Point, i64>,
    beacons: &HashSet<&Point>,
    row: i64,
) -> i64 {
    let mut covered = HashSet::<i64>::new();

    for (sensor_coords, soi) in sensors {
        if (row - sensor_coords.y).abs() > *soi {
            continue;
        }
        let cover_len = *soi - (row - sensor_coords.y).abs();
        for cover_x in (sensor_coords.x - cover_len)..=(sensor_coords.x + cover_len) {
            covered.insert(cover_x);
        }
    }
    let beacons_in_row = beacons.iter().filter(|beacon| beacon.y == row).count();
    (covered.len() - beacons_in_row).try_into().unwrap()
}

fn get_beacon_set(sensor_beacon_pairs: &[(Point, Point)]) -> HashSet<&Point> {
    let result: HashSet<_> = sensor_beacon_pairs
        .iter()
        .map(|(_, beacon)| beacon)
        .collect();
    result
}

fn get_sensor_sois(sensor_beacon_pairs: &[(Point, Point)]) -> HashMap<Point, i64> {
    let mut result = HashMap::<Point, i64>::new();

    for (sensor_coords, beacon_coords) in sensor_beacon_pairs.iter() {
        let soi = (*beacon_coords - *sensor_coords).len();
        result.insert(*sensor_coords, soi);
    }

    result
}

fn get_sensor_beacon_pairs_from_input() -> Vec<(Point, Point)> {
    let input_file = File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);
    let mut result = Vec::<(Point, Point)>::new();
    for wrapped_line in input_buf_read.lines() {
        let line = wrapped_line.unwrap();
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        let sensor_x = words[2]
            .trim_start_matches("x=")
            .trim_end_matches(',')
            .parse::<i64>()
            .unwrap();
        let sensor_y = words[3]
            .trim_start_matches("y=")
            .trim_end_matches(':')
            .parse::<i64>()
            .unwrap();
        let beacon_x = words[8]
            .trim_start_matches("x=")
            .trim_end_matches(',')
            .parse::<i64>()
            .unwrap();
        let beacon_y = words[9].trim_start_matches("y=").parse::<i64>().unwrap();
        result.push((
            Point::with_coords(sensor_x, sensor_y),
            Point::with_coords(beacon_x, beacon_y),
        ));
        // Sensor at x=X1, y=Y1: closest beacon is at x=X2, y=Y2 -> "x=X1," (at 2), "y=Y1:" (at 3), "x=X2," (at 8), "y=Y2" (at 9)
    }

    result
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn with_coords(x: i64, y: i64) -> Point {
        Point { x, y }
    }
    fn len(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}
