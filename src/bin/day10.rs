use std::collections::HashSet;

use aoc_2024::{Grid, Point};
use itertools::Itertools;

fn climb(grid: &Grid<u8>, pos: Point, summits: &mut HashSet<Point>) -> usize {
    if grid[pos] == 9 {
        summits.insert(pos);
        return 1;
    }

    let next_positions = pos
        .orthogonal()
        .filter(|&new_pos| grid[new_pos] == grid[pos] + 1)
        .collect_vec();
    if next_positions.is_empty() {
        return 0;
    }

    next_positions
        .iter()
        .map(|&pos| climb(grid, pos, summits))
        .sum()
}

fn main() {
    let grid: Grid<u8> = include_str!("../../inputs/day10.txt").parse().unwrap();
    let (mut p1, mut p2) = (0, 0);
    for (_, pos) in grid.items().filter(|&(elem, _)| *elem == 0) {
        let mut summits = HashSet::new();
        p2 += climb(&grid, pos, &mut summits);
        p1 += summits.len();
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
