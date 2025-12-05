pub fn p3() {
    let input = include_str!("./input.txt").trim();
    let mut total = 0;
    let count = 12;
    for line in input.lines() {
        let digits: Vec<_> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let mut numbers = vec![];
        let mut next_idx = 0;
        for i in 0..count {
            let range = &digits[next_idx..=(digits.len() - count + i).min(digits.len())];
            let mut max_d = 0;
            let mut max_idx = 0;
            for (idx, &d) in range.iter().enumerate() {
                if d > max_d {
                    max_d = d;
                    max_idx = idx;
                }
            }
            numbers.push(max_d);
            next_idx += max_idx + 1;
        }

        let mut num = 0;
        for (idx, n) in numbers.iter().enumerate() {
            num += n * 10usize.pow((numbers.len() - idx - 1) as u32);
        }
        total += num;
    }

    println!("{}", total)
}
