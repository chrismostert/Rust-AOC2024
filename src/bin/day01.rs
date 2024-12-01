use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day01.txt");
    let mut freqs: HashMap<usize, usize> = HashMap::new();

    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .flat_map(|line| {
            line.split_whitespace()
                .map(|elem| elem.parse().unwrap())
                .collect_tuple()
                .inspect(|&(_, right)| {
                    *freqs.entry(right).or_default() += 1;
                })
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    println!(
        "Part 1: {}",
        left.iter()
            .zip(right.iter())
            .map(|(&l, &r)| l.abs_diff(r))
            .sum::<usize>()
    );
    println!(
        "Part 2: {}",
        left.iter()
            .map(|&val| val * freqs.get(&val).unwrap_or(&0))
            .sum::<usize>()
    );
}
