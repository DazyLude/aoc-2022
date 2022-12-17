#![allow(dead_code, unused_variables)]

use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let mut sim = CellularRopetomata::new();
    read_and_apply_input(&mut sim);
    println!("pt1: {}", sim.visited.len());

    let mut sims2 = CellRopeChain::with_size(10);
    read_and_apply_input(&mut sims2);
    println!("pt2: {}", sims2.visited.len());
}

fn read_and_apply_input(sim: &mut dyn Simulation) {
    let input_file = File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);

    input_buf_read.lines().for_each(|wrapped_line| {
        let line = wrapped_line.unwrap();
        let mut command_iter = line.split_ascii_whitespace();
        let direction = match command_iter.next() {
            Some("U") => Direction::Up,
            Some("D") => Direction::Down,
            Some("R") => Direction::Right,
            Some("L") => Direction::Left,
            _ => panic!("Unknown command code"),
        };
        let distance = command_iter.next().unwrap().parse::<i64>().unwrap();
        for _ in 0..distance {
            sim.move_head(direction);
            sim.tick();
        }
    });
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    fn is_adjacent_to(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn drag_tail(&self, tail: &mut Point) {
        let mut dx = self.x - tail.x;
        if dx.abs() == 2 {
            dx /= 2;
        }
        tail.x += dx;
        let mut dy = self.y - tail.y;
        if dy.abs() == 2 {
            dy /= 2;
        }
        tail.y += dy;
    }

    fn move_to(&mut self, where_to: Direction) {
        match where_to {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

trait Simulation {
    fn tick(&mut self);
    fn move_head(&mut self, direction: Direction);
}

struct CellularRopetomata {
    tail: Point,
    head: Point,
    visited: HashSet<Point>,
}

impl CellularRopetomata {
    fn new() -> CellularRopetomata {
        CellularRopetomata {
            tail: Point::new(),
            head: Point::new(),
            visited: HashSet::from([Point::new()]),
        }
    }
}

impl Simulation for CellularRopetomata {
    fn move_head(&mut self, which_direction: Direction) {
        self.head.move_to(which_direction);
    }

    fn tick(&mut self) {
        if !self.head.is_adjacent_to(&self.tail) {
            self.head.drag_tail(&mut self.tail);
            self.visited.insert(self.tail.clone());
        }
    }
}

struct CellRopeChain {
    rope: Vec<Point>,
    visited: HashSet<Point>,
}

impl CellRopeChain {
    fn with_size(length: usize) -> CellRopeChain {
        let mut clc = CellRopeChain {
            rope: Vec::<Point>::new(),
            visited: HashSet::<Point>::new(),
        };
        for _ in 0..length {
            clc.rope.push(Point::new());
        }
        clc
    }
}

impl Simulation for CellRopeChain {
    fn move_head(&mut self, which_direction: Direction) {
        self.rope[0].move_to(which_direction);
    }

    fn tick(&mut self) {
        for rope_i in 1..self.rope.len() {
            let head = &self.rope[rope_i - 1].clone();
            if self.rope[rope_i].is_adjacent_to(head) {
                break;
            }
            head.drag_tail(&mut self.rope[rope_i]);
        }
        self.visited.insert(self.rope.last().unwrap().clone());
    }
}
