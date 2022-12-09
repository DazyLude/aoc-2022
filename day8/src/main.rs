// #![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let mut tree_grid = Grid::<u8>::new();
    read_input_to_grid(&mut tree_grid);

    let normal_grid =
        GridNavigator::<u8>::for_grid_in_direction(&mut tree_grid, Direction::LeftToRight)
            .make_visibility_mask();

    let right_to_left_grid =
        GridNavigator::<u8>::for_grid_in_direction(&mut tree_grid, Direction::RightToLeft)
            .make_visibility_mask();

    let top_to_bottom_grid =
        GridNavigator::<u8>::for_grid_in_direction(&mut tree_grid, Direction::TopToBottom)
            .make_visibility_mask();

    let bottom_to_top_grid =
        GridNavigator::<u8>::for_grid_in_direction(&mut tree_grid, Direction::BottomToTop)
            .make_visibility_mask();

    // println!("{:?}", normal_grid);
    // println!("{:?}", right_to_left_grid);
    // println!("{:?}", top_to_bottom_grid);
    // println!("{:?}", bottom_to_top_grid);

    let pt1 = (normal_grid + right_to_left_grid + top_to_bottom_grid + bottom_to_top_grid)
        .grid
        .iter()
        .map(|&is_visible| i32::from(is_visible))
        .sum::<i32>();

    println!("{pt1} trees are visible");
}

fn read_input_to_grid(grid: &mut Grid<u8>) {
    let input_file = File::open("input").expect("Could not open input file");
    let mut input_buf_read = BufReader::new(input_file).lines();

    let mut width: usize = 0;

    input_buf_read
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .enumerate()
        .for_each(|(i, tree)| {
            width = i;
            grid.grid.push(tree.to_digit(10).unwrap() as u8);
        });

    input_buf_read.for_each(|wrapped_line| {
        wrapped_line.unwrap().chars().for_each(|tree| {
            grid.grid.push(tree.to_digit(10).unwrap() as u8);
        })
    });

    grid.set_width(width + 1);
}

#[derive(Debug, Clone)]
enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[derive(Debug, Clone)]
struct Grid<T> {
    pub grid: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    fn new() -> Grid<T> {
        Grid {
            grid: Vec::<T>::new(),
            width: 1,
        }
    }

    fn set_width(&mut self, new_width: usize) {
        self.width = new_width;
    }

    fn get_height(&self) -> usize {
        self.grid.len() / self.width
    }

    fn _is_valid(&self) -> bool {
        self.width * self.get_height() == self.grid.len()
    }
}

impl std::ops::Add for Grid<bool> {
    type Output = Self;

    fn add(self, other: Grid<bool>) -> Self {
        let mut result = Grid::<bool>::new();
        result.set_width(self.width);

        for index in 0..self.grid.len() {
            result.grid.push(self.grid[index] || other.grid[index]);
        }

        result
    }
}

struct GridNavigator<'a, T>
where
    T: Copy,
{
    nav: &'a mut Grid<T>,
    direction: Direction,
    current_item: usize,
}

impl<T: std::marker::Copy> GridNavigator<'_, T> {
    fn for_grid_in_direction(grid: &mut Grid<T>, direction: Direction) -> GridNavigator<T> {
        GridNavigator {
            nav: grid,
            direction,
            current_item: 0,
        }
    }

    fn row(&self) -> usize {
        self.current_item / self.nav.width
    }

    fn col(&self) -> usize {
        self.current_item % self.nav.width
    }

    fn get_current_index(&self) -> usize {
        match &self.direction {
            Direction::LeftToRight => self.current_item,
            Direction::RightToLeft => self.nav.width - self.col() - 1 + self.row() * self.nav.width,
            Direction::TopToBottom => self.col() * self.nav.width + self.row(),
            Direction::BottomToTop => {
                (self.nav.get_height() - self.col() - 1) * self.nav.width + self.row()
            }
        }
    }

    fn get_item(&self) -> Option<T> {
        if self.current_item >= self.nav.grid.len() {
            return None;
        }
        self.nav.grid.get(self.get_current_index()).copied()
    }

    fn set_item(&mut self, item: T) {
        let index = self.get_current_index();
        *self.nav.grid.get_mut(index).unwrap() = item
    }

    fn is_new_big_item(&self) -> bool {
        match &self.direction {
            Direction::LeftToRight => self.get_current_index() % self.nav.width == 0,
            Direction::RightToLeft => {
                self.get_current_index() % self.nav.width == self.nav.width - 1
            }
            Direction::TopToBottom => self.get_current_index() / self.nav.width == 0,
            Direction::BottomToTop => {
                self.get_current_index() / self.nav.width == self.nav.get_height() - 1
            }
        }
    }
}

trait VisibilityMaskable {
    fn make_visibility_mask(&mut self) -> Grid<bool>;
}

impl VisibilityMaskable for GridNavigator<'_, u8> {
    fn make_visibility_mask(&mut self) -> Grid<bool> {
        let mut visibility_mask = Grid::<bool>::new();
        self.nav
            .grid
            .iter()
            .for_each(|_| visibility_mask.grid.push(false));
        visibility_mask.set_width(self.nav.width);

        let mut visibility_mask_navigator =
            GridNavigator::for_grid_in_direction(&mut visibility_mask, self.direction.clone());

        let mut max_in_big_item = 0;

        while self.current_item < self.nav.grid.len() {
            if self.is_new_big_item() || self.get_item().unwrap() > max_in_big_item {
                max_in_big_item = self.get_item().unwrap();
                visibility_mask_navigator.set_item(true);
            }

            visibility_mask_navigator.current_item += 1;
            self.current_item += 1;
        }

        visibility_mask
    }
}

impl<T: std::marker::Copy> Iterator for GridNavigator<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let item = self.get_item();
        self.current_item += 1;
        item
    }
}
