fn main() {
    let mut cc = include_str!("../../inputs/6.txt")
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .fold(vec![0usize; 9], |mut cc, r| {
            cc[r] += 1;
            cc
        });

    for _ in 0..256 {
        let cc0 = cc[0];
        cc[0] = cc[1];
        cc[1] = cc[2];
        cc[2] = cc[3];
        cc[3] = cc[4];
        cc[4] = cc[5];
        cc[5] = cc[6];
        cc[6] = cc[7] + cc0;
        cc[7] = cc[8];
        cc[8] = cc0;
    }

    println!("{}", cc.iter().sum::<usize>())
}
