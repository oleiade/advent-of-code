use std::ops::{Add, Not};

use itertools::Itertools;

#[aoc_generator(day8)]
fn input_generator_part2(input: &str) -> Forest {
    let mut forest = Forest::new(input.lines().count(), input.lines().next().unwrap().len());

    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, column)| {
            let ix = x as isize;
            let iy = y as isize;
            forest.set_tree(Vector2 { x: ix, y: iy }, column as usize - '0' as usize)
        })
    });

    forest
}

#[aoc(day8, part1)]
fn solve_part1(input: &Forest) -> usize {
    // We iterate through each tree one by one
    (0..input.width)
        .cartesian_product(0..input.height)
        .map(|(x, y)| {
            let x_pos = x as isize;
            let y_pos = y as isize;
            input.is_visible(Vector2 { x: x_pos, y: y_pos })
        })
        .filter(|&is_visible| is_visible)
        .count()
}

#[aoc(day8, part2)]
fn solve_part2(input: &Forest) -> usize {
    (0..input.width)
        .cartesian_product(0..input.height)
        .map(|(x, y)| {
            let x_pos = x as isize;
            let y_pos = y as isize;
            input.viewing_distance(Vector2 { x: x_pos, y: y_pos })
        })
        .max()
        .unwrap()
}

struct Forest {
    height: usize,
    width: usize,
    trees: Vec<usize>,
}

impl Forest {
    fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            trees: vec![0; height * width],
        }
    }

    fn tree(&self, position: Vector2) -> usize {
        self.trees[position.y as usize * self.width + position.x as usize]
    }

    fn set_tree(&mut self, position: Vector2, value: usize) {
        self.trees[position.y as usize * self.width + position.x as usize] = value;
    }

    fn scan(&self, from: Vector2, direction: Direction) -> Vec<usize> {
        let (to, direction_vec) = match direction {
            Direction::Up => (Vector2 { x: from.x, y: 0 }, UP),
            Direction::Down => (
                Vector2 {
                    x: from.x,
                    y: (self.height - 1) as isize,
                },
                DOWN,
            ),
            Direction::Left => (Vector2 { x: 0, y: from.y }, LEFT),
            Direction::Right => (
                Vector2 {
                    x: (self.width - 1) as isize,
                    y: from.y,
                },
                RIGHT,
            ),
        };

        let mut current = from;
        let mut scanned = Vec::new();
        while current != to {
            current = current + direction_vec;
            scanned.push(self.tree(current));
        }

        scanned
    }

    fn is_edge(&self, position: Vector2) -> bool {
        position.x == 0
            || position.x == self.width as isize - 1
            || position.y == 0
            || position.y == self.height as isize - 1
    }

    fn is_visible(&self, position: Vector2) -> bool {
        if self.is_edge(position) {
            return true;
        }

        let tree_height = self.tree(position);

        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        for direction in directions.iter() {
            let scanned = self.scan(position, *direction);
            let hidden = scanned.iter().any(|x| *x >= tree_height);

            if hidden.not() {
                return true;
            }
        }

        false
    }

    fn viewing_distance(&self, position: Vector2) -> usize {
        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        let mut total = 0;
        let tree_height = self.tree(position);
        for direction in directions.iter() {
            let mut direction_total = 0;

            let scanned = self.scan(position, *direction);
            for height in scanned {
                if height >= tree_height {
                    direction_total += 1;
                    break;
                }

                direction_total += 1;
            }

            if direction_total == 0 {
                return 0;
            }

            if total == 0 {
                total = 1;
            }

            total *= direction_total;
        }

        total
    }
}

const UP: Vector2 = Vector2 { x: 0, y: -1 };
const DOWN: Vector2 = Vector2 { x: 0, y: 1 };
const LEFT: Vector2 = Vector2 { x: -1, y: 0 };
const RIGHT: Vector2 = Vector2 { x: 1, y: 0 };

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Vector2> for Direction {
    fn from(vector: Vector2) -> Self {
        match vector {
            UP => Direction::Up,
            DOWN => Direction::Down,
            LEFT => Direction::Left,
            RIGHT => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Vector2 {
    x: isize,
    y: isize,
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
