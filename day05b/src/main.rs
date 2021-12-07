fn main() {
    let count = include_str!("../../inputs/5.txt")
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let (x1, y1) = left.split_once(",").unwrap();
            let (x2, y2) = right.split_once(",").unwrap();

            let x1 = x1.parse::<usize>().unwrap();
            let y1 = y1.parse::<usize>().unwrap();
            let x2 = x2.parse::<usize>().unwrap();
            let y2 = y2.parse::<usize>().unwrap();

            if x1 > x2 {
                (x2, y2, x1, y1)
            } else if x1 == x2 && y1 > y2 {
                (x1, y2, x2, y1)
            } else {
                (x1, y1, x2, y2)
            }
        })
        .fold(vec![vec![0; 1000]; 1000], |mut map, (x1, y1, x2, y2)| {
            if y1 == y2 {
                (x1..=x2).for_each(|i| map[i][y1] += 1);
            } else if x1 == x2 {
                (y1..=y2).for_each(|i| map[x1][i] += 1);
            } else if y1 < y2 {
                (0..=(x2 - x1)).for_each(|i| map[x1 + i][y1 + i] += 1)
            } else {
                (0..=(x2 - x1)).for_each(|i| map[x1 + i][y1 - i] += 1)
            }

            map
        })
        .iter()
        .flatten()
        .filter(|c| **c >= 2)
        .count();

    println!("{}", count);
}
