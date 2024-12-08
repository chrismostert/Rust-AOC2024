use aoc_2024::{Grid, Point, DIAGONAL, DOWNLEFT, DOWNRIGHT, UPLEFT, UPRIGHT};
use std::str::FromStr;

fn n_word_hits(grid: &Grid<char>, point: Point, word: &[u8]) -> usize {
    DIAGONAL
        .into_iter()
        .filter(|&direction| {
            (0..word.len()).all(|i| grid[point + direction * i as u32] == word[i] as char)
        })
        .count()
}

fn is_cross_mas(grid: &Grid<char>, point: Point) -> bool {
    grid[point] == 'A'
        && [
            (point + DOWNLEFT, point + UPRIGHT),
            (point + DOWNRIGHT, point + UPLEFT),
        ]
        .iter()
        .all(|&(a, b)| [['M', 'S'], ['S', 'M']].contains(&[grid[a], grid[b]]))
}

fn main() {
    let input: Grid<char> = Grid::from_str(include_str!("../../inputs/day04.txt")).unwrap();
    let (p1, p2) = input.coords().fold((0, 0), |(p1, p2), point| {
        (
            p1 + n_word_hits(&input, point, b"XMAS"),
            p2 + is_cross_mas(&input, point) as usize,
        )
    });

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
