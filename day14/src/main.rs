// #![allow(dead_code, unused_variables)]
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut occupied_space = HashSet::<Point>::new();
    get_rock_from_input(&mut occupied_space);
    let lowest_rock_point = occupied_space
        .iter()
        .reduce(|lowest, current| {
            if lowest.y > current.y {
                lowest
            } else {
                current
            }
        })
        .unwrap()
        .y;
    let mut occupied_space_for_pt2 = occupied_space.clone();

    let mut sand_count = 0;
    while drop_sand(&mut occupied_space, lowest_rock_point).is_some() {
        sand_count += 1;
    }

    println!("pt1: {sand_count}");

    sand_count = 0;
    while let Some(new_sand_just_dropped) =
        drop_sand_on_floor(&occupied_space_for_pt2, lowest_rock_point)
    {
        occupied_space_for_pt2.insert(new_sand_just_dropped);
        sand_count += 1;
    }

    println!("pt2: {sand_count}");
}

fn drop_sand_on_floor(world: &HashSet<Point>, lowest_rock_point: i64) -> Option<Point> {
    let mut sand_block = Point::with_coords(500, 0);
    if world.contains(&sand_block) {
        return None;
    }

    let floor_z = lowest_rock_point + 1;

    while sand_block.y < floor_z {
        if !world.contains(&sand_block.get_below()) {
            sand_block = sand_block.get_below();
            continue;
        }
        if !world.contains(&sand_block.get_left_and_below()) {
            sand_block = sand_block.get_left_and_below();
            continue;
        }
        if !world.contains(&sand_block.get_right_and_below()) {
            sand_block = sand_block.get_right_and_below();
            continue;
        }
        break;
    }
    Some(sand_block)
}

fn drop_sand(world: &mut HashSet<Point>, kill_z: i64) -> Option<Point> {
    let mut sand_block = Point::with_coords(500, 0);
    while sand_block.y < kill_z {
        if !world.contains(&sand_block.get_below()) {
            sand_block = sand_block.get_below();
            continue;
        }
        if !world.contains(&sand_block.get_left_and_below()) {
            sand_block = sand_block.get_left_and_below();
            continue;
        }
        if !world.contains(&sand_block.get_right_and_below()) {
            sand_block = sand_block.get_right_and_below();
            continue;
        }
        world.insert(sand_block);
        return Some(sand_block);
    }
    None
}

fn get_rock_from_input(world: &mut HashSet<Point>) {
    let input_file = File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);

    let fill_between = |key_points: Vec<Point>| -> Vec<Point> {
        let mut result = Vec::<Point>::new();

        for point_index in 0..key_points.len() - 1 {
            let delta = key_points[point_index + 1] - key_points[point_index];
            let (start_x, end_x) = match delta.x.cmp(&0) {
                std::cmp::Ordering::Equal => (0, 1),
                std::cmp::Ordering::Less => (delta.x + 1, 1),
                std::cmp::Ordering::Greater => (0, delta.x),
            };

            let (start_y, end_y) = match delta.y.cmp(&0) {
                std::cmp::Ordering::Equal => (0, 1),
                std::cmp::Ordering::Less => (delta.y + 1, 1),
                std::cmp::Ordering::Greater => (0, delta.y),
            };

            for dx in start_x..end_x {
                for dy in start_y..end_y {
                    result.push(key_points[point_index] + Point::with_coords(dx, dy));
                }
            }
        }
        result.push(*key_points.last().unwrap());
        result
    };

    for wrapped_line in input_buf_read.lines() {
        let key_points = wrapped_line
            .unwrap()
            .split_terminator(" -> ")
            .map(|str_coords| {
                let coords = str_coords
                    .split_terminator(',')
                    .map(|point_str| point_str.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                Point::with_coords(coords[0], coords[1])
            })
            .collect::<Vec<Point>>();

        for point in fill_between(key_points).iter() {
            world.insert(*point);
        }
    }
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

    fn get_below(&self) -> Point {
        Point::with_coords(self.x, self.y + 1)
    }

    fn get_left_and_below(&self) -> Point {
        Point::with_coords(self.x - 1, self.y + 1)
    }

    fn get_right_and_below(&self) -> Point {
        Point::with_coords(self.x + 1, self.y + 1)
    }
}
