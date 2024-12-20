use std::ops::RangeInclusive;

use aoc_2024::{
    dijkstra::{dijkstra, orthogonal_grid_moves, ShortestPath},
    Grid, Point,
};

fn find_n_cheats(shortest_path: &ShortestPath<Point>, cheat_range: RangeInclusive<u64>) -> u64 {
    shortest_path
        .path
        .iter()
        .enumerate()
        .flat_map(|(idx, point)| {
            shortest_path.path[idx + 1..]
                .iter()
                .map(move |p2| (point, p2))
        })
        .filter(|&(p1, p2)| cheat_range.contains(&p1.manhattan(*p2)))
        .map(|(p1, p2)| {
            shortest_path.distances[p2] - shortest_path.distances[p1] - p1.manhattan(*p2)
        })
        .filter(|&cheat_dist| cheat_dist >= 100)
        .count() as u64
}

fn main() {
    let grid = include_str!("../../inputs/day20.txt")
        .parse::<Grid<char>>()
        .unwrap();

    let shortest_path = dijkstra(
        grid.find('S').unwrap(),
        grid.find('E').unwrap(),
        orthogonal_grid_moves(&grid),
    )
    .unwrap();

    println!("Part 1: {}", find_n_cheats(&shortest_path, 2..=2));
    println!("Part 2: {}", find_n_cheats(&shortest_path, 2..=20));
}
