fn main() {
    let data = include_str!("../../inputs/3.txt");

    let bins: Vec<_> = data.split_whitespace().collect();
    let len = bins.iter().map(|bin| bin.len()).max().unwrap();
    let mut zeros = vec![0; len];
    let mut ones = vec![0; len];

    bins.iter().for_each(|bin| {
        for (i, c) in bin.chars().enumerate() {
            match c {
                '0' => zeros[i] += 1,
                '1' => ones[i] += 1,
                _ => {}
            }
        }
    });

    let mut gamma = 0u64;
    let mut epsilon = 0u64;

    for (n0, n1) in zeros.into_iter().zip(ones.into_iter()) {
        if n0 >= n1 {
            gamma = gamma << 1;
            epsilon = (epsilon << 1) + 1;
        } else {
            gamma = (gamma << 1) + 1;
            epsilon = epsilon << 1;
        }
    }

    println!("{}", gamma * epsilon);
}
