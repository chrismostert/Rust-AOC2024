use aoc_2024::{Grid, Point, LEFT, RIGHT};

fn try_move(grid: &mut Grid<char>, to_move: Point, dir: Point) -> bool {
    let new_pos = to_move + dir;
    match grid[new_pos] {
        c @ (']' | '[') => {
            if try_move(grid, new_pos + if c == '[' { RIGHT } else { LEFT }, dir)
                && try_move(grid, new_pos, dir)
            {
                return try_move(grid, to_move, dir);
            }
            false
        }
        'O' => {
            if try_move(grid, new_pos, dir) {
                return try_move(grid, to_move, dir);
            }
            false
        }
        '.' => {
            grid[new_pos] = grid[to_move];
            grid[to_move] = '.';
            true
        }
        _ => false,
    }
}

fn score(grid: &Grid<char>) -> u64 {
    grid.items().fold(0, |score, (elem, point)| match elem {
        'O' | '[' => score + (point.0 as u64) + 100 * (point.1 as u64),
        _ => score,
    })
}

fn main() {
    let (grid, dirs) = include_str!("../../inputs/day15.txt")
        .split_once("\n\n")
        .unwrap();
    let mut small_grid: Grid<char> = grid.parse().unwrap();
    let mut big_grid: Grid<char> = grid
        .chars()
        .map(|c| match c {
            '#' => "##",
            'O' => "[]",
            '.' => "..",
            '@' => "@.",
            _ => "\n",
        })
        .collect::<String>()
        .parse()
        .unwrap();

    for dir in dirs
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c.try_into().unwrap())
    {
        let robot = small_grid.find('@').unwrap();
        let robot_big = big_grid.find('@').unwrap();
        let prev = big_grid.clone();

        try_move(&mut small_grid, robot, dir);
        if !try_move(&mut big_grid, robot_big, dir) {
            // Undo if move was invalid
            big_grid = prev;
        }
    }

    println!("Part 1: {}", score(&small_grid));
    println!("Part 2: {}", score(&big_grid));
}
