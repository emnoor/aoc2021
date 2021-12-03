use itertools::Itertools;

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .tuple_windows()
            .map(|(a, b)| if a < b { 1 } else { 0 })
            .sum::<i64>()
    );
}
