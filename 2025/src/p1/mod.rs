pub fn p1() {
    let input = include_str!("./input.txt").trim();
    let mut count = 0;
    let mut dial = 50;
    for line in input.lines() {
        let turns: isize = line[1..].parse().unwrap();
        match line.chars().nth(0) {
            Some('L') => {
                for _ in 0..turns {
                    dial -= 1;
                    match dial {
                        0 => count += 1,
                        -1 => dial = 99,
                        _ => (),
                    }
                }
            }
            Some('R') => {
                for _ in 0..turns {
                    dial += 1;
                    match dial {
                        100 => {
                            dial = 0;
                            count += 1;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    println!("{}", count)
}
