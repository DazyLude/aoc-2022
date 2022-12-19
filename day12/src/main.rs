use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let height_map = HeightMap::from_input();
    println!("pt1: {:?}", height_map.find_shortest_path());
    println!("pt2: {:?}", height_map.find_shortest_path_down());
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTION_SLICE: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<i16>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl HeightMap {
    fn from_input() -> HeightMap {
        let mut map = Vec::<Vec<i16>>::new();
        let input_file = File::open("input").expect("Could not open input file");
        let input_buf_read = BufReader::new(input_file);

        let mut point = (0, 0);
        let mut start = (0, 0);
        let mut end = (0, 0);

        for wrapped_line in input_buf_read.lines() {
            map.push(Vec::<i16>::new());
            for height_code in wrapped_line.unwrap().chars() {
                match height_code {
                    'S' => {
                        start = point;
                        map.last_mut().unwrap().push(0)
                    }
                    'E' => {
                        end = point;
                        map.last_mut().unwrap().push(25)
                    }
                    _ => map
                        .last_mut()
                        .unwrap()
                        .push((height_code as u8 - b'a') as i16),
                }
                point.1 += 1;
            }
            point.0 += 1;
            point.1 = 0;
        }
        HeightMap { map, start, end }
    }

    fn find_shortest_path(&self) -> Option<u64> {
        let mut visited = HashMap::<(usize, usize), u64>::new();
        let mut buffer = HashSet::<(usize, usize)>::new();
        let mut turn_counter = 0;

        let try_go_somewhere =
            |(x, y): (usize, usize), where_to: &Direction| -> Option<(usize, usize)> {
                let current_height = self.map[x][y];
                match where_to {
                    Direction::Up if x > 0 => {
                        if current_height - self.map[x - 1][y] < 2 {
                            Some((x - 1, y))
                        } else {
                            None
                        }
                    }
                    Direction::Left if y > 0 => {
                        if current_height - self.map[x][y - 1] < 2 {
                            Some((x, y - 1))
                        } else {
                            None
                        }
                    }
                    Direction::Down if x + 1 < self.map.len() => {
                        if current_height - self.map[x + 1][y] < 2 {
                            Some((x + 1, y))
                        } else {
                            None
                        }
                    }
                    Direction::Right if y + 1 < self.map[0].len() => {
                        if current_height - self.map[x][y + 1] < 2 {
                            Some((x, y + 1))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            };

        buffer.insert(self.end);

        while !visited.contains_key(&self.start) && !buffer.is_empty() {
            let mut new_buffer = HashSet::<(usize, usize)>::new();
            for new_place in buffer.drain() {
                visited.insert(new_place, turn_counter);
                for there in DIRECTION_SLICE.iter() {
                    if let Some(go_there) = try_go_somewhere(new_place, there) {
                        if !visited.contains_key(&go_there) {
                            new_buffer.insert(go_there);
                        }
                    }
                }
            }
            turn_counter += 1;
            buffer = new_buffer;
        }
        visited.get(&self.start).copied()
    }

    fn find_shortest_path_down(&self) -> Option<u64> {
        let mut visited = HashMap::<(usize, usize), u64>::new();
        let mut buffer = HashSet::<(usize, usize)>::new();
        let mut turn_counter = 0;

        let try_go_somewhere =
            |(x, y): (usize, usize), where_to: &Direction| -> Option<(usize, usize)> {
                let current_height = self.map[x][y];
                match where_to {
                    Direction::Up if x > 0 => {
                        if current_height - self.map[x - 1][y] < 2 {
                            Some((x - 1, y))
                        } else {
                            None
                        }
                    }
                    Direction::Left if y > 0 => {
                        if current_height - self.map[x][y - 1] < 2 {
                            Some((x, y - 1))
                        } else {
                            None
                        }
                    }
                    Direction::Down if x + 1 < self.map.len() => {
                        if current_height - self.map[x + 1][y] < 2 {
                            Some((x + 1, y))
                        } else {
                            None
                        }
                    }
                    Direction::Right if y + 1 < self.map[0].len() => {
                        if current_height - self.map[x][y + 1] < 2 {
                            Some((x, y + 1))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            };

        buffer.insert(self.end);

        'big_l: while !visited.contains_key(&self.start) && !buffer.is_empty() {
            let mut new_buffer = HashSet::<(usize, usize)>::new();
            for new_place in buffer.drain() {
                if self.map[new_place.0][new_place.1] == 0 {
                    break 'big_l;
                }
                visited.insert(new_place, turn_counter);
                for there in DIRECTION_SLICE.iter() {
                    if let Some(go_there) = try_go_somewhere(new_place, there) {
                        if !visited.contains_key(&go_there) {
                            new_buffer.insert(go_there);
                        }
                    }
                }
            }
            turn_counter += 1;
            buffer = new_buffer;
        }
        Some(turn_counter)
    }
}
