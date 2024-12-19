use std::collections::HashSet;

use cached::proc_macro::cached;

#[cached(key = "&str", convert = "{towel_sequence}")]
fn is_valid(towel_sequence: &'static str, towels: &HashSet<&str>, max_towel_size: usize) -> bool {
    let max_idx = (max_towel_size + 1).min(towel_sequence.len() + 1);
    if towel_sequence.is_empty() {
        return true;
    }
    for idx in 0..max_idx {
        if towels.contains(&towel_sequence[0..idx])
            && is_valid(&towel_sequence[idx..], towels, max_towel_size)
        {
            return true;
        }
    }
    false
}

fn main() {
    let (stock, targets) = include_str!("../../inputs/day19.txt")
        .split_once("\n\n")
        .unwrap();
    let stock: HashSet<&str> = stock.split(", ").collect();
    let targets = targets.lines();
    let max_towel_size = stock.iter().map(|t| t.len()).max().unwrap();

    let p1 = targets
        .filter(|target| is_valid(target, &stock, max_towel_size))
        .count();

    dbg!(p1);
}
