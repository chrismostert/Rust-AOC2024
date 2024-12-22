use aoc_2024::{Grid, Point};
use cached::proc_macro::cached;
use itertools::Itertools;

fn paths(keypad: &Grid<char>, from: char, to: char) -> Vec<String> {
    let avoid = keypad.find('.').unwrap();
    let from = keypad.find(from).unwrap();
    let Point(dx, dy) = keypad.find(to).unwrap() - from;
    let hor = if dx > 0 { ">" } else { "<" };
    let vert = if dy > 0 { "v" } else { "^" };

    let mut paths = Vec::new();

    // Horizontal first
    if dx.unsigned_abs() > 0 && from + Point(dx, 0) != avoid {
        paths.push(format!(
            "{}{}A",
            hor.repeat(dx.unsigned_abs()),
            vert.repeat(dy.unsigned_abs())
        ));
    }

    // Vertical first
    if dy.unsigned_abs() > 0 && from + Point(0, dy) != avoid {
        paths.push(format!(
            "{}{}A",
            vert.repeat(dy.unsigned_abs()),
            hor.repeat(dx.unsigned_abs())
        ));
    }

    if paths.is_empty() {
        vec![String::from("A")]
    } else {
        paths
    }
}

// We can cache this because for each higher level, all robots have to
// be in position 'A' after a key is pressed (propagated through the layers)
#[cached(
    key = "String",
    convert = r#"{ format!("{}{}{}", level, limit, code) }"#
)]
fn n_presses(code: &str, level: u8, limit: u8, pads: [&Grid<char>; 2]) -> u64 {
    let keypad = if level == 0 { pads[0] } else { pads[1] };
    // We press the buttons with our own fingers
    if level == limit {
        code.len() as u64
    // Or we let the robot do it for us
    } else {
        code.chars()
            .fold((0, 'A'), |(len, current_key), next_key| {
                (
                    len + paths(keypad, current_key, next_key)
                        .iter()
                        .map(|code| n_presses(code, level + 1, limit, pads))
                        .min()
                        .unwrap(),
                    next_key,
                )
            })
            .0
    }
}

fn main() {
    let codes = include_str!("../../inputs/day21.txt").lines().collect_vec();
    let (numpad, dirpad): (Grid<char>, Grid<char>) = (
        "789\n456\n123\n.0A".parse().unwrap(),
        ".^A\n<v>".parse().unwrap(),
    );
    let complexity = |codes: &[&str], n_keypads| -> u64 {
        codes
            .iter()
            .map(|code| {
                code[0..code.len() - 1].parse::<u64>().unwrap()
                    * n_presses(code, 0, n_keypads, [&numpad, &dirpad])
            })
            .sum()
    };

    println!("Part 1: {}", complexity(&codes, 3));
    println!("Part 2: {}", complexity(&codes, 26));
}
