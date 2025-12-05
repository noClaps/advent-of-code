pub fn p5() {
    let input = include_str!("./input.txt").trim();
    let mut input = input.lines();

    let mut ranges = vec![];
    while let Some(line) = input.next()
        && !line.is_empty()
    {
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        let (start, end): (usize, usize) = (start.parse().unwrap(), end.parse().unwrap());

        ranges.push((start, end));
    }
    let mut new_ranges = process_ranges(&ranges);
    while new_ranges != ranges {
        ranges = new_ranges;
        new_ranges = process_ranges(&ranges);
    }
    let ranges = new_ranges;

    let mut count = 0;
    for range in ranges {
        count += range.1 - range.0 + 1 // +1 because inclusive (3-1=2, len(1,2,3) = 3)
    }
    println!("{}", count);
}

fn process_ranges(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut new_ranges = vec![];
    'outer: for &(start, end) in ranges {
        for &(s, e) in ranges {
            if (s, e) == (start, end) {
                continue;
            }
            if start < s && end > e {
                // s..e = 3..4
                // start..end = 1..5
                // new: 1..5
                new_ranges.push((start, end));
                continue 'outer;
            }
            if start < s && (s..=e).contains(&end) {
                // s..e = 3..5
                // start..end = 2..4
                // new: 2..5
                new_ranges.push((start, e));
                continue 'outer;
            }
            if (s..=e).contains(&start) && end > e {
                // s..e = 3..5
                // start..end = 4..6
                // new: 3..6
                new_ranges.push((s, end));
                continue 'outer;
            }
            if (s..=e).contains(&start) && (s..=e).contains(&end) {
                continue 'outer;
            }
        }
        if !new_ranges.contains(&(start, end)) {
            new_ranges.push((start, end));
        }
    }

    new_ranges
}
