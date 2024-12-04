use aoc_2024::CharGrid;
use std::str::FromStr;

use itertools::Itertools;

fn n_word_hits(grid: &CharGrid, (x, y): (isize, isize), word: &[u8]) -> usize {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(dx, dy)| {
            (0..word.len())
                .all(|i| grid[(x + dx * i as isize, y + dy * i as isize)] == word[i] as char)
        })
        .count()
}

fn is_cross_mas(grid: &CharGrid, (x, y): (isize, isize)) -> bool {
    grid[(x, y)] == 'A'
        && [
            ((x - 1, y - 1), (x + 1, y + 1)),
            ((x + 1, y - 1), (x - 1, y + 1)),
        ]
        .iter()
        .all(|&(a, b)| [['M', 'S'], ['S', 'M']].contains(&[grid[a], grid[b]]))
}

fn main() {
    let input = CharGrid::from_str(include_str!("../../inputs/day04.txt")).unwrap();
    let (p1, p2) = input.keys().fold((0, 0), |(p1, p2), &coord| {
        (
            p1 + n_word_hits(&input, coord, b"XMAS"),
            p2 + is_cross_mas(&input, coord) as usize,
        )
    });

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
