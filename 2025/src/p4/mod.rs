#[derive(PartialEq, Clone)]
enum Cell {
    Roll,
    Empty,
}

pub fn p4() {
    let input = include_str!("./input.txt").trim();
    let mut input: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => Cell::Roll,
                    '.' => Cell::Empty,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut total_removed = 0;
    loop {
        let (updated, removed) = remove_rolls(input);
        if removed == 0 {
            break;
        }
        input = updated;
        total_removed += removed;
    }

    println!("{}", total_removed)
}

fn remove_rolls(input: Vec<Vec<Cell>>) -> (Vec<Vec<Cell>>, usize) {
    let mut updated_input = input.clone();

    let mut count = 0;
    let row_count = input.len();
    let col_count = input[0].len();
    for r in 0..input.len() {
        for c in 0..input[r].len() {
            if input[r][c] != Cell::Roll {
                continue;
            }

            let mut roll_neighbors = 0;
            // check north
            if r > 0 && input[r - 1][c] == Cell::Roll {
                roll_neighbors += 1
            }
            // check northeast
            if r > 0 && c + 1 < col_count && input[r - 1][c + 1] == Cell::Roll {
                roll_neighbors += 1
            }
            // check east
            if c + 1 < col_count && input[r][c + 1] == Cell::Roll {
                roll_neighbors += 1
            }
            // check southeast
            if r + 1 < row_count && c + 1 < col_count && input[r + 1][c + 1] == Cell::Roll {
                roll_neighbors += 1
            }
            // check south
            if r + 1 < row_count && input[r + 1][c] == Cell::Roll {
                roll_neighbors += 1
            }
            // check southwest
            if r + 1 < row_count && c > 0 && input[r + 1][c - 1] == Cell::Roll {
                roll_neighbors += 1
            }
            // check west
            if c > 0 && input[r][c - 1] == Cell::Roll {
                roll_neighbors += 1
            }
            // check northwest
            if r > 0 && c > 0 && input[r - 1][c - 1] == Cell::Roll {
                roll_neighbors += 1
            }

            if roll_neighbors < 4 {
                updated_input[r][c] = Cell::Empty;
                count += 1
            }
        }
    }

    (updated_input, count)
}
