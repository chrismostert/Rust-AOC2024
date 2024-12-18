use aoc_2024::{Grid, Point, RIGHT};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: Point,
    direction: Point,
    cost: u64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    grid: &Grid<char>,
    start: (Point, Point),
    dest: Point,
) -> Option<(State, HashMap<(Point, Point), HashSet<(Point, Point)>>)> {
    let mut dists: HashMap<(Point, Point), u64> = HashMap::new();
    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    let mut prev: HashMap<(Point, Point), HashSet<(Point, Point)>> = HashMap::new();

    dists.insert(start, 0);
    frontier.push(State {
        position: start.0,
        direction: start.1,
        cost: 0,
    });

    while let Some(State {
        cost,
        position,
        direction,
    }) = frontier.pop()
    {
        // Destination reached!
        if position == dest {
            return Some((
                State {
                    cost,
                    position,
                    direction,
                },
                prev,
            ));
        }
        // There was a better way to get here
        if cost > *dists.get(&(position, direction)).unwrap_or(&u64::MAX) {
            continue;
        }

        let mut next_states = vec![
            State {
                position,
                direction: direction.clockwise(),
                cost: cost + 1000,
            },
            State {
                position,
                direction: direction.clockwise().clockwise().clockwise(),
                cost: cost + 1000,
            },
        ];
        if grid[position + direction] == '.' || grid[position + direction] == 'E' {
            next_states.push(State {
                position: position + direction,
                direction,
                cost: cost + 1,
            })
        }

        for State {
            cost: new_cost,
            position: new_position,
            direction: new_direction,
        } in next_states
        {
            let prev_cost = *dists
                .get(&(new_position, new_direction))
                .unwrap_or(&u64::MAX);
            if new_cost <= prev_cost {
                if new_cost < prev_cost {
                    prev.insert((new_position, new_direction), HashSet::new());
                }
                prev.get_mut(&(new_position, new_direction))
                    .unwrap()
                    .insert((position, direction));
                dists.insert((new_position, new_direction), new_cost);
                frontier.push(State {
                    cost: new_cost,
                    position: new_position,
                    direction: new_direction,
                })
            }
        }
    }
    None
}

fn main() {
    let grid: Grid<char> = include_str!("../../inputs/day16.txt").parse().unwrap();
    let dest = grid.find('E').unwrap();
    let (end_state, prev) = dijkstra(&grid, (grid.find('S').unwrap(), RIGHT), dest).unwrap();

    let mut visited: HashSet<Point> = HashSet::new();
    let mut frontier: Vec<(Point, Point)> = Vec::from([(end_state.position, end_state.direction)]);
    while let Some(point) = frontier.pop() {
        visited.insert(point.0);
        for &new_point in prev.get(&point).unwrap_or(&HashSet::new()) {
            frontier.push(new_point);
        }
    }

    println!("Part 1: {}", end_state.cost);
    println!("Part 2: {}", visited.len());
}
