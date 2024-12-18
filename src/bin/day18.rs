use aoc_2024::dijkstra::{dijkstra, State};
use aoc_2024::{Grid, Point};
use itertools::Itertools;

fn next_states(grid: &Grid<char>) -> impl Fn(&State<Point>) -> Vec<State<Point>> + use<'_> {
    |s: &State<Point>| {
        s.position
            .orthogonal()
            .filter(|&new_p| grid[new_p] == '.')
            .map(|new_p| State {
                position: new_p,
                cost: s.cost + 1,
            })
            .collect_vec()
    }
}

fn main() {
    let mut input = include_str!("../../inputs/day18.txt")
        .split(|c: char| !c.is_ascii_digit())
        .filter(|w| !w.is_empty())
        .map(|d| d.parse::<isize>().unwrap())
        .tuples()
        .map(|(x, y)| Point(x, y));

    let mut grid = Grid::init(71, 71, '.');
    for _ in 0..1024 {
        grid[input.next().unwrap()] = '#';
    }

    let p1 = dijkstra(Point(0, 0), Point(70, 70), next_states(&grid)).unwrap();

    let mut p2 = None;
    let mut prev_path = p1.path;
    for point in input {
        grid[point] = '#';
        if prev_path.contains(&point) {
            match dijkstra(Point(0, 0), Point(70, 70), next_states(&grid)) {
                None => {
                    p2 = Some(point);
                    break;
                }
                Some(shortest_path) => prev_path = shortest_path.path,
            }
        }
    }

    println!("Part 1: {}", p1.cost);
    println!("Part 2: {},{}", p2.unwrap().0, p2.unwrap().1);
}
