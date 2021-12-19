fn main() {
    println!(
        "{}",
        include_str!("../../inputs/8.txt")
            .lines()
            .map(|line| {
                let (_, output) = line.split_once(" | ").unwrap();
                output.split_ascii_whitespace().map(|x| {
                    let len = x.len();
                    if len == 2 || len == 4 || len == 3 || len == 7 {
                        1
                    } else {
                        0
                    }
                })
                // .sum::<usize>()
            })
            .flatten()
            .sum::<usize>()
    );
}
