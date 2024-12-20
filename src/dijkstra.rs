use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use itertools::Itertools;

use crate::{Grid, Point};

pub fn orthogonal_grid_moves(grid: &Grid<char>) -> impl Fn(&State<Point>) -> Vec<State<Point>> + '_ {
    |state: &State<Point>| {
        state
            .position
            .orthogonal()
            .filter(|&new_pos| ['.', 'E'].contains(&grid[new_pos]))
            .map(|new_pos| State {
                position: new_pos,
                cost: state.cost + 1,
            })
            .collect_vec()
    }
}

#[derive(PartialEq, Eq)]
pub struct State<T> {
    pub position: T,
    pub cost: u64,
}

pub struct ShortestPath<T> {
    pub cost: u64,
    pub path: Vec<T>,
    pub distances: HashMap<T, u64>
}

impl<T: Eq + PartialEq> Ord for State<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T: Eq + PartialEq> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn get_path<T: Clone + Eq + Hash>(prevs: &HashMap<T, T>, dest: T) -> Vec<T> {
    let mut path = vec![];
    let mut frontier = vec![dest];

    while let Some(node) = frontier.pop() {
        if let Some(prev) = prevs.get(&node) {
            frontier.push(prev.clone());
        }
        path.insert(0, node);
    }

    path
}

pub fn dijkstra<T: Eq + PartialEq + Hash + Clone>(
    start: T,
    dest: T,
    next_states: impl Fn(&State<T>) -> Vec<State<T>>,
) -> Option<ShortestPath<T>> {
    let mut dists: HashMap<T, u64> = HashMap::new();
    let mut frontier: BinaryHeap<State<T>> = BinaryHeap::new();
    let mut prevs: HashMap<T, T> = HashMap::new();

    dists.insert(start.clone(), 0);
    frontier.push(State {
        position: start,
        cost: 0,
    });

    while let Some(state) = frontier.pop() {
        if state.position == dest {
            return Some(ShortestPath {
                cost: state.cost,
                path: get_path(&prevs, dest),
                distances: dists
            });
        }
        if state.cost > *dists.get(&state.position).unwrap_or(&u64::MAX) {
            continue;
        }

        for next_state in next_states(&state) {
            let prev_cost = *dists.get(&next_state.position).unwrap_or(&u64::MAX);
            if next_state.cost < prev_cost {
                dists.insert(next_state.position.clone(), next_state.cost);
                prevs.insert(next_state.position.clone(), state.position.clone());
                frontier.push(State {
                    cost: next_state.cost,
                    position: next_state.position,
                })
            }
        }
    }
    None
}
