fn main() {
    let mut x = 0;
    let mut y = 0;

    include_str!("../input.txt").lines().for_each(|line| {
        let (cmd, n) = line.split_once(" ").unwrap();
        let n = n.parse::<i64>().unwrap();
        match cmd {
            "forward" => {
                x += n;
            }
            "down" => {
                y += n;
            }
            "up" => {
                y -= n;
            }
            _ => {}
        }
    });

    println!("{}", x * y);
}
