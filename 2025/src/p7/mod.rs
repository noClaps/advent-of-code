// i couldnt figure out part 2, so i got help for my solution from here:
// https://www.reddit.com/r/adventofcode/comments/1pgnmou/2025_day_7_lets_visualize/

pub fn p7() {
    let input = include_str!("./input.txt").trim();

    let mut input = input.lines();
    let first = input.next().unwrap().chars();
    let mut beams = vec![];
    for c in first {
        if c == 'S' {
            beams.push(1)
        } else {
            beams.push(0);
        }
    }

    for line in input {
        if !line.contains('^') {
            continue;
        }
        // line contains ^
        for (i, c) in line.char_indices() {
            if c != '^' {
                continue;
            }

            // at splitter, c == '^'
            beams[i - 1] += beams[i];
            beams[i + 1] += beams[i];
            beams[i] = 0;
        }
    }
    println!("{}", beams.iter().sum::<usize>());
}
