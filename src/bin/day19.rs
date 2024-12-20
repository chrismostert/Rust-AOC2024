use std::collections::HashSet;

use cached::proc_macro::cached;

#[cached(key = "&str", convert = "{target}")]
fn n_valid(target: &'static str, stock: &HashSet<&str>, max_towel_size: usize) -> u64 {
    if target.is_empty() {
        return 1;
    }
    (0..=max_towel_size.min(target.len()))
        .map(|idx| {
            if stock.contains(&target[0..idx]) {
                return n_valid(&target[idx..], stock, max_towel_size);
            }
            0
        })
        .sum()
}

fn main() {
    let (stock, targets) = include_str!("../../inputs/day19.txt")
        .split_once("\n\n")
        .unwrap();
    let stock: HashSet<&str> = stock.split(", ").collect();
    let targets: Vec<&str> = targets.lines().collect();
    let max_towel_size = stock.iter().map(|t| t.len()).max().unwrap();

    let p1 = targets
        .iter()
        .filter(|target| n_valid(target, &stock, max_towel_size) > 0)
        .count();
    let p2: u64 = targets
        .iter()
        .map(|target| n_valid(target, &stock, max_towel_size))
        .sum();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
