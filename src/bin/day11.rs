use cached::proc_macro::cached;

#[cached]
fn count_stones(stone_no: u64, blinks: u8) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if stone_no == 0 {
        return count_stones(1, blinks - 1);
    }
    let digits = stone_no.ilog(10) + 1;
    if digits % 2 == 0 {
        return count_stones(stone_no / 10u64.pow(digits / 2), blinks - 1)
            + count_stones(stone_no % 10u64.pow(digits / 2), blinks - 1);
    }
    count_stones(stone_no * 2024, blinks - 1)
}

fn main() {
    let input: Vec<u64> = include_str!("../../inputs/day11.txt")
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let p1: u64 = input.iter().map(|&num| count_stones(num, 25)).sum();
    let p2: u64 = input.iter().map(|&num| count_stones(num, 75)).sum();
    
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
