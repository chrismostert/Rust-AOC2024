use itertools::Itertools;

fn solve(input: &[(u64, Vec<u64>)], ops: &[impl Fn(u64, u64) -> u64]) -> u64 {
    input
        .iter()
        .filter_map(|(target, eq)| recurse(eq, ops, eq[0], *target))
        .sum()
}

fn recurse(eq: &[u64], ops: &[impl Fn(u64, u64) -> u64], current: u64, target: u64) -> Option<u64> {
    return match (eq.len() == 1, current == target, current > target) {
        // (equation_ended, target_reached, target_exceeded)
        (true, true, _) => Some(target),
        (true, false, _) => None,
        (false, _, true) => None,
        _ => ops
            .iter()
            .map(|op| recurse(&eq[1..], ops, op(current, eq[1]), target))
            .find(|res| res.is_some())
            .unwrap_or_default(),
    };
}

fn main() {
    let input: Vec<(u64, Vec<u64>)> = include_str!("../../inputs/day07.txt")
        .lines()
        .map(|line| {
            let (result, equation) = line.split_once(": ").unwrap();
            (
                result.parse::<u64>().unwrap(),
                equation
                    .split_whitespace()
                    .map(|elem| elem.parse::<u64>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let ops = [
        |a, b| a + b,
        |a, b| a * b,
        |a, b| {
            let mut pow = 10;
            while b >= pow {
                pow *= 10
            }
            a * pow + b
        },
    ];

    println!("Part 1: {}", solve(&input, &ops[..2]));
    println!("Part 2: {}", solve(&input, &ops));
}
