use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

fn main() {
    let (ordering, jobs) = include_str!("../../inputs/day05.txt")
        .split_once("\n\n")
        .unwrap();

    let ordering: HashSet<(u32, u32)> = ordering
        .lines()
        .map(|line| {
            line.split('|')
                .map(|e| e.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let jobs: Vec<Vec<u32>> = jobs
        .lines()
        .map(|line| line.split(',').map(|e| e.parse::<u32>().unwrap()).collect())
        .collect();

    let (p1, p2) = jobs.iter().fold((0, 0), |(p1, p2), job| {
        let sorted_job = job
            .iter()
            .copied()
            .sorted_by(|&a, &b| {
                if ordering.contains(&(a, b)) {
                    return Ordering::Less;
                }
                Ordering::Greater
            })
            .collect_vec();

        if job == &sorted_job {
            return (p1 + job[job.len() / 2], p2);
        }
        (p1, p2 + sorted_job[sorted_job.len() / 2])
    });

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
