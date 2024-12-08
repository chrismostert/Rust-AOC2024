use std::collections::{HashMap, HashSet};

use aoc_2024::{Grid, Point};
use itertools::Itertools;
type Beacons = HashMap<char, Vec<Point>>;

fn solve(grid: &Grid<char>, beacons: &Beacons, p2: bool) -> usize {
    let mut coords: HashSet<Point> = HashSet::new();
    for (point_1, point_2) in beacons.values().flat_map(|coords| {
        coords
            .iter()
            .cartesian_product(coords.iter())
            .filter(|(c1, c2)| c1 != c2)
            .map(|(p1, p2)| (*p1, *p2))
    }) {
        let delta = point_2 - point_1;
        if !p2 {
            if grid[point_1 - delta] != '\0' {
                coords.insert(point_1 - delta);
            }
        } else {
            let mut mul = 0;
            while grid[point_1 + delta * mul] != '\0' {
                coords.insert(point_1 + delta * mul);
                mul += 1
            }
        }
    }
    coords.len()
}

fn main() {
    let grid = include_str!("../../inputs/day08.txt")
        .parse::<Grid<char>>()
        .unwrap();
    let beacons: Beacons = grid.items().filter(|(&beacon, _)| beacon != '.').fold(
        HashMap::new(),
        |mut acc, (beacon, coord)| {
            acc.entry(*beacon).or_default().push(coord);
            acc
        },
    );

    println!("Part 1: {}", solve(&grid, &beacons, false));
    println!("Part 2: {}", solve(&grid, &beacons, true));
}
