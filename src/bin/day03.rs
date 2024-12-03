use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/day03.txt");

    let (_, (p1, p2)) = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(input)
        .fold((1, (0, 0)), |(state, (p1, p2)), capt| match &capt[0][..3] {
            "do(" => (1, (p1, p2)),
            "don" => (0, (p1, p2)),
            _ => {
                let mul = capt[1].parse::<usize>().unwrap() * capt[2].parse::<usize>().unwrap();
                (state, (p1 + mul, p2 + state * mul))
            }
        });

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
