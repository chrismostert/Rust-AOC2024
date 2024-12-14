use std::{cmp::Ordering, collections::HashSet};

use aoc_2024::Point;
use itertools::Itertools;

fn safety_value(dims: Point, robots: &[(Point, Point)]) -> u64 {
    let (x_border, y_border) = (dims.0 / 2, dims.1 / 2);
    let (mut tl, mut tr, mut bl, mut br) = (0, 0, 0, 0);
    for (point, _) in robots {
        match (point.0.cmp(&x_border), point.1.cmp(&y_border)) {
            (Ordering::Less, Ordering::Less) => tl += 1,
            (Ordering::Less, Ordering::Greater) => bl += 1,
            (Ordering::Greater, Ordering::Less) => tr += 1,
            (Ordering::Greater, Ordering::Greater) => br += 1,
            _ => (),
        }
    }
    tl * tr * bl * br
}

fn print_configuration(dims: Point, configuration: &[(Point, Point)]) {
    let positions: HashSet<Point> = configuration.iter().map(|&(pos, _)| pos).collect();
    for y in 0..dims.1 {
        for x in 0..dims.0 {
            if positions.contains(&Point(x, y)) {
                print!("#");
            } else {
                print!(" ")
            }
        }
        println!();
    }
}

fn main() {
    let dims = Point(101, 103);
    let mut robots = include_str!("../../inputs/day14.txt")
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|w| !w.is_empty())
        .map(|d| d.parse::<isize>().unwrap())
        .tuples()
        .map(|(x, y, vx, vy)| (Point(x, y), Point(vx, vy)))
        .collect_vec();

    let p1 = robots
        .iter()
        .map(|&(pos, vel)| ((pos + vel * 100) % dims, vel))
        .collect_vec();

    // If a bunch of robots clump together, we estimate that the safety value will be lowest
    // since a bunch of robots will be in a same quadrant
    let mut lowest = u64::MAX;
    let mut lowest_loop = 0;
    let mut configuration = robots.clone();
    for i in 0..(dims.0 * dims.1) {
        let safety = safety_value(dims, &robots);
        if safety < lowest {
            lowest = safety;
            lowest_loop = i;
            configuration = robots.clone()
        }
        robots = robots
            .into_iter()
            .map(|(pos, vel)| ((pos + vel) % dims, vel))
            .collect();
    }

    // Check visually to make sure we found the christmas tree
    // and because it is fun.
    print_configuration(dims, &configuration);

    println!("Part 1: {}", safety_value(dims, &p1));
    println!("Part 2: {}", lowest_loop);
}
