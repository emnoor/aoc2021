fn main() {
    let bins: Vec<_> = include_str!("../input.txt").split_whitespace().collect();
    let number_of_bits = bins.iter().map(|bin| bin.len()).max().unwrap();

    let mut o2_bins = bins.clone();
    for idx in 0..number_of_bits {
        if o2_bins.len() == 1 {
            break;
        }

        let mut zeros = 0;
        let mut ones = 0;
        for bin in &o2_bins {
            match bin.as_bytes()[idx] as char {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => {}
            }
        }

        let expected_digit = if ones >= zeros { '1' } else { '0' };
        o2_bins = o2_bins
            .into_iter()
            .filter(|bin| bin.as_bytes()[idx] as char == expected_digit)
            .collect();
    }

    let mut co2_bins = bins.clone();
    for idx in 0..number_of_bits {
        if co2_bins.len() == 1 {
            break;
        }

        let mut zeros = 0;
        let mut ones = 0;
        for bin in &co2_bins {
            match bin.as_bytes()[idx] as char {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => {}
            }
        }

        let expected_digit = if zeros <= ones { '0' } else { '1' };
        co2_bins = co2_bins
            .into_iter()
            .filter(|bin| bin.as_bytes()[idx] as char == expected_digit)
            .collect();
    }

    println!(
        "{}",
        u64::from_str_radix(o2_bins[0], 2).unwrap() * u64::from_str_radix(co2_bins[0], 2).unwrap()
    );
}
