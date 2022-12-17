use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut tree_grid = Vec::<Vec<i8>>::new();
    read_input_to_grid(&mut tree_grid);
    let mut nav = GridNavigator::new(&tree_grid);
    let pt1: i64 = calc_seen_from_outside(&mut nav);

    let pt2 = get_max_score(&mut nav);

    println!("pt1: {pt1}; pt2: {pt2}");
}

fn read_input_to_grid(grid: &mut Vec<Vec<i8>>) {
    let input_file = File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);

    input_buf_read.lines().for_each(|wrapped_line| {
        let row: Vec<i8> = wrapped_line
            .unwrap()
            .chars()
            .map(|char| char.to_digit(10).unwrap() as i8)
            .collect();
        grid.push(row);
    });
}

struct GridNavigator<'a, T> {
    row: usize,
    col: usize,
    grid: &'a Vec<Vec<T>>,
    out_of_bounds_flag: bool,
}

enum GoToOption {
    NextRow,
    RowEnd,
    RowBegin,
    NextCol,
    ColEnd,
    ColBegin,
    Begin,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<T> GridNavigator<'_, T>
where
    T: Copy,
{
    fn new(grid: &Vec<Vec<T>>) -> GridNavigator<T> {
        GridNavigator {
            row: 0,
            col: 0,
            grid,
            out_of_bounds_flag: false,
        }
    }

    fn go_to(&mut self, where_to: GoToOption) {
        self.out_of_bounds_flag = false;
        match where_to {
            GoToOption::RowEnd => self.col = self.grid[0].len() - 1,
            GoToOption::RowBegin => self.col = 0,
            GoToOption::NextRow => self.row += 1,
            GoToOption::NextCol => self.col += 1,
            GoToOption::ColEnd => self.row = self.grid.len() - 1,
            GoToOption::ColBegin => self.row = 0,
            GoToOption::Begin => {
                self.col = 0;
                self.row = 0;
            }
        };
    }

    fn go(&mut self, direction: Direction) {
        match direction {
            Direction::Up if self.row > 0 => self.row -= 1,
            Direction::Down => self.row += 1,
            Direction::Left if self.col > 0 => self.col -= 1,
            Direction::Right => self.col += 1,
            _ => self.out_of_bounds_flag = true,
        }
    }

    fn get(&mut self) -> Option<T> {
        if self.out_of_bounds_flag {
            return None;
        }
        Some(*self.grid.get(self.row)?.get(self.col)?)
    }

    fn set_position(&mut self, new_row: usize, new_col: usize) {
        self.out_of_bounds_flag = false;
        self.col = new_col;
        self.row = new_row;
    }

    fn get_position(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

fn get_max_score(nav: &mut GridNavigator<i8>) -> i64 {
    let mut max_so_far = 0;

    nav.go_to(GoToOption::Begin);
    while nav.get().is_some() {
        while nav.get().is_some() {
            let new_score = calc_scenic_score(nav);
            if new_score > max_so_far {
                max_so_far = new_score;
            }
            nav.go(Direction::Right);
        }
        nav.go_to(GoToOption::NextRow);
        nav.go_to(GoToOption::RowBegin);
    }

    max_so_far
}

fn calc_scenic_score(nav: &mut GridNavigator<i8>) -> i64 {
    let mut score = 1;
    let mut count = 0;
    let self_height = nav.get().unwrap();
    let (save_row, save_col) = nav.get_position();

    let mut look_there = |look_where: Direction| {
        nav.go(look_where);
        count = 0;
        while let Some(height) = nav.get() {
            count += 1;
            if height >= self_height {
                break;
            }
            nav.go(look_where);
        }
        score *= count;
        nav.set_position(save_row, save_col);
    };

    look_there(Direction::Up);
    look_there(Direction::Down);
    look_there(Direction::Left);
    look_there(Direction::Right);

    score
}

fn calc_seen_from_outside(nav: &mut GridNavigator<i8>) -> i64 {
    let mut seen_mask = Vec::<Vec<bool>>::new();
    for row in 0..nav.grid.len() {
        seen_mask.push(Vec::<bool>::new());
        for _ in 0..nav.grid[0].len() {
            seen_mask[row].push(false);
        }
    }

    nav.go_to(GoToOption::Begin);
    while nav.get().is_some() {
        let mut last_max = -1;
        while let Some(value) = nav.get() {
            if value > last_max {
                seen_mask[nav.row][nav.col] = true;
                last_max = value;
            }
            nav.go(Direction::Right);
        }
        nav.go_to(GoToOption::NextRow);
        nav.go_to(GoToOption::RowBegin);
    }

    nav.go_to(GoToOption::Begin);
    nav.go_to(GoToOption::RowEnd);
    while nav.get().is_some() {
        let mut last_max = -1;
        while let Some(value) = nav.get() {
            if value > last_max {
                seen_mask[nav.row][nav.col] = true;
                last_max = value;
            }
            nav.go(Direction::Left);
        }
        nav.go_to(GoToOption::NextRow);
        nav.go_to(GoToOption::RowEnd);
    }

    nav.go_to(GoToOption::Begin);
    while nav.get().is_some() {
        let mut last_max = -1;
        while let Some(value) = nav.get() {
            if value > last_max {
                seen_mask[nav.row][nav.col] = true;
                last_max = value;
            }
            nav.go(Direction::Down);
        }
        nav.go_to(GoToOption::NextCol);
        nav.go_to(GoToOption::ColBegin);
    }

    nav.go_to(GoToOption::Begin);
    nav.go_to(GoToOption::ColEnd);
    while nav.get().is_some() {
        let mut last_max = -1;
        while let Some(value) = nav.get() {
            if value > last_max {
                seen_mask[nav.row][nav.col] = true;
                last_max = value;
            }
            nav.go(Direction::Up);
        }
        nav.go_to(GoToOption::NextCol);
        nav.go_to(GoToOption::ColEnd);
    }

    let count = seen_mask
        .iter()
        .map(|row| row.iter().map(|is_seen| *is_seen as i64).sum::<i64>())
        .sum();

    count
}
