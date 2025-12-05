pub fn p2() {
    let input = include_str!("./input.txt").trim();
    let ranges: Vec<(usize, usize)> = input
        .split(',')
        .map(|r| r.split_once('-').unwrap())
        .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        .collect();

    let mut id_sum = 0;
    for (start, end) in ranges {
        'outer: for i in start..=end {
            let i_str = i.to_string();
            for j in 1..=i_str.len() / 2 {
                if i_str.len() % j != 0 {
                    continue;
                }
                let slice = &i_str[0..j];
                let repetitions = i_str.len() / j;
                if i_str == slice.repeat(repetitions) {
                    id_sum += i;
                    continue 'outer;
                }
            }
        }
    }

    println!("{}", id_sum)
}
