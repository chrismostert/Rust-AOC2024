use itertools::Itertools;

// Cramer's rule for two equations
fn solve(x1: i64, y1: i64, x2: i64, y2: i64, z1: i64, z2: i64) -> i64 {
    let d = x1 * y2 - x2 * y1;
    let a = (z1 * y2 - x2 * z2) / d;
    let b = (x1 * z2 - z1 * y1) / d;
    if x1 * a + x2 * b == z1 && y1 * a + y2 * b == z2 {
        return a * 3 + b;
    }
    0
}

fn main() {
    let mut p1 = 0;
    let mut p2 = 0;
    for (x1, y1, x2, y2, z1, z2) in include_str!("../../inputs/day13.txt")
        .split(|c: char| !c.is_ascii_digit())
        .filter(|spl| !spl.is_empty())
        .map(|d| d.parse::<i64>().unwrap())
        .tuples()
    {
        p1 += solve(x1, y1, x2, y2, z1, z2);
        p2 += solve(x1, y1, x2, y2, z1 + 10000000000000, z2 + 10000000000000);
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
