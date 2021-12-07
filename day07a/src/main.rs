fn main() {
    let positions: Vec<i32> = include_str!("../../inputs/7.txt")
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!(
        "{}",
        (1..positions.len())
            .map(|new_position| cost(&positions, new_position as i32))
            .min()
            .unwrap()
    );
}

fn cost(positions: &[i32], new_position: i32) -> i32 {
    positions.iter().map(|x| (new_position - x).abs()).sum()
}
