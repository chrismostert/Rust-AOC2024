use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn next_secret(num: i64) -> i64 {
    let a = ((num * 64) ^ num) % 16777216;
    let b = ((a / 32) ^ a) % 16777216;
    ((b * 2048) ^ b) % 16777216
}

fn n_secrets(num: i64, n: u32) -> Vec<i64> {
    (0..n).fold(vec![num], |mut secrets, _| {
        secrets.push(next_secret(*secrets.last().unwrap()));
        secrets
    })
}

fn main() {
    let (p1, totals) = include_str!("../../inputs/day22.txt")
        .lines()
        .map(|d| n_secrets(d.parse().unwrap(), 2000))
        .fold((0, HashMap::<_, i64>::new()), |(p1, mut totals), seq| {
            let mut seen: HashSet<(i64, i64, i64, i64)> = HashSet::new();
            seq.iter()
                .tuple_windows()
                .map(|(a, b)| (b % 10 - a % 10, b % 10))
                .tuple_windows()
                .for_each(|((a, _), (b, _), (c, _), (d, price))| {
                    if !seen.contains(&(a, b, c, d)) {
                        *totals.entry((a, b, c, d)).or_default() += price;
                        seen.insert((a, b, c, d));
                    }
                });
            (p1 + seq.last().unwrap(), totals)
        });

    println!("Part 1: {}", p1);
    println!("Part 2: {}", totals.values().max().unwrap());
}
