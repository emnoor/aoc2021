fn main() {
    let map: Vec<Vec<u32>> = include_str!("../../inputs/9.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let m = map.len();
    let n = map[0].len();

    let mut sum_risk_level = 0;

    for i in 0..m {
        for j in 0..n {
            if (i == 0 || map[i][j] < map[i - 1][j])
                && (i == n - 1 || map[i][j] < map[i + 1][j])
                && (j == 0 || map[i][j] < map[i][j - 1])
                && (j == n - 1 || map[i][j] < map[i][j + 1])
            {
                sum_risk_level += 1 + map[i][j];
            }
        }
    }

    println!("{}", sum_risk_level);
}
