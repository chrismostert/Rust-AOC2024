use itertools::Itertools;

fn is_safe(report: &[isize]) -> bool {
    report
        .windows(2)
        .map(|w| w[1] - w[0])
        .tuple_windows()
        .all(|(a, b)| a * b > 0 && a.abs() <= 3 && b.abs() <= 3)
}

fn main() {
    let reports: Vec<Vec<isize>> = include_str!("../../inputs/day02.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect()
        })
        .collect();

    println!(
        "Part 1: {}",
        reports.iter().filter(|report| is_safe(report)).count()
    );
    println!(
        "Part 2: {}",
        reports
            .iter()
            .filter(|report| {
                (0..report.len())
                    .map(|idx| [&report[0..idx], &report[idx + 1..]].concat())
                    .any(|report| is_safe(&report))
            })
            .count()
    );
}
