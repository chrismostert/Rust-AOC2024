use std::{collections::HashSet, hash::Hash, str::FromStr};

use aoc_2024::{CharGrid, Direction};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    position: (isize, isize),
    direction: Direction,
}

impl Guard {
    fn step(mut self, grid: &CharGrid, obstruction: Option<(isize, isize)>) -> Option<Self> {
        let (dx, dy) = self.direction.as_coord();
        let new_pos = (self.position.0 + dx, self.position.1 + dy);
        match (grid[new_pos], Some(new_pos) == obstruction) {
            ('#', _) | (_, true) => {
                self.direction = self.direction.turn_right();
                self.step(grid, obstruction)
            }
            ('.', false) | ('^', _) => {
                self.position = new_pos;
                Some(self)
            }
            _ => None,
        }
    }

    fn step_positions(
        mut self,
        grid: &CharGrid,
        obstruction: Option<(isize, isize)>,
    ) -> Option<HashSet<(isize, isize)>> {
        let mut been = HashSet::from([self]);
        let mut seen_positions = HashSet::from([self.position]);
        while let Some(pos) = self.step(grid, obstruction) {
            // Loop detected
            if been.contains(&pos) {
                return None;
            }
            seen_positions.insert(pos.position);
            been.insert(pos);
            self = pos
        }
        Some(seen_positions)
    }
}

fn main() {
    let grid = CharGrid::from_str(include_str!("../../inputs/day06.txt")).unwrap();
    let guard = Guard {
        position: grid.find_char('^').unwrap(),
        direction: Direction::Up,
    };

    let guard_path = guard.step_positions(&grid, None).unwrap();

    let p1 = guard_path.len();
    let p2 = guard_path
        .into_par_iter()
        .map(|obstruction| guard.step_positions(&grid, Some(obstruction)))
        .filter(|res| res.is_none())
        .count();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
