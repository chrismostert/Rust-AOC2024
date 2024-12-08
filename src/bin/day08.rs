use std::collections::{HashMap, HashSet};

use aoc_2024::Grid;
use itertools::Itertools;
type Coord = (isize, isize);
type Beacons = HashMap<char, Vec<Coord>>;

fn solve(grid: &Grid<char>, beacons: &Beacons, p2: bool) -> usize {
    let mut coords: HashSet<Coord> = HashSet::new();
    for ((x1, y1), (x2, y2)) in beacons.values().flat_map(|coords| {
        coords
            .iter()
            .cartesian_product(coords.iter())
            .filter(|(c1, c2)| c1 != c2)
    }) {
        let (dx, dy) = (x2 - x1, y2 - y1);
        if !p2 {
            if grid[(x1 - dx, y1 - dy)] != '\0' {
                coords.insert((x1 - dx, y1 - dy));
            }
        } else {
            let mut mul = 0;
            while grid[(x1 + mul * dx, y1 + mul * dy)] != '\0' {
                coords.insert((x1 + mul * dx, y1 + mul * dy));
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
