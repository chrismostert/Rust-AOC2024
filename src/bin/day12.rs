use std::collections::HashSet;

use aoc_2024::{Grid, Point, DOWN, DOWNLEFT, DOWNRIGHT, LEFT, RIGHT, UP, UPLEFT, UPRIGHT};

fn floodfill(grid: &Grid<char>, visited: &mut HashSet<Point>, c: char, point: Point) -> (u64, u64) {
    let mut reachable = vec![point];

    let mut area = 0;
    let mut perimeter = 0;
    let mut outer_corners: u64 = 0;
    let mut inner_corners: u64 = 0;

    while let Some(point) = reachable.pop() {
        if visited.contains(&point) {
            continue;
        }
        visited.insert(point);

        area += 1;
        perimeter += point
            .orthogonal()
            .filter(|&adj_point| grid[adj_point] != c)
            .count() as u64;
        outer_corners += [[UP, RIGHT], [RIGHT, DOWN], [DOWN, LEFT], [LEFT, UP]]
            .into_iter()
            .filter(|&[d1, d2]| grid[point + d1] != c && grid[point + d2] != c)
            .count() as u64;
        inner_corners += [
            [UPRIGHT, UP, RIGHT],
            [DOWNRIGHT, DOWN, RIGHT],
            [DOWNLEFT, DOWN, LEFT],
            [UPLEFT, UP, LEFT],
        ]
        .into_iter()
        .filter(|&[d_diff, d_same, d2_same]| {
            grid[point + d_diff] != c && grid[point + d_same] == c && grid[point + d2_same] == c
        })
        .count() as u64;

        for reachable_point in point
            .orthogonal()
            .filter(|&adj_point| grid[adj_point] == c && !visited.contains(&adj_point))
        {
            reachable.push(reachable_point);
        }
    }

    (area * perimeter, area * (inner_corners + outer_corners))
}

fn main() {
    let input: Grid<char> = include_str!("../../inputs/day12.txt").parse().unwrap();

    let mut visited = HashSet::new();
    let (mut p1, mut p2) = (0, 0);

    for (&c, point) in input.items() {
        if visited.contains(&point) {
            continue;
        }
        let (d1, d2) = floodfill(&input, &mut visited, c, point);
        p1 += d1;
        p2 += d2;
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
