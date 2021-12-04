use itertools::Itertools;

fn main() {
    println!(
        "{}",
        include_str!("../../inputs/1.txt")
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .tuple_windows()
            .map(|(a, _, _, b)| if a < b { 1 } else { 0 })
            .sum::<i64>()
    );
}
