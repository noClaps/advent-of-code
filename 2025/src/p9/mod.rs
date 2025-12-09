// added rayon to make it faster. i tried to run it without rayon, and im sure it wouldve worked
// eventually without it, but i dont have the patience for that. there's also probably a better
// algo for this that's like orders of magnitude faster, but i couldnt come up with anything

use std::{fmt::Debug, usize};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn p9() {
    let input = include_str!("./input.txt").trim();

    let mut min_coords = Coords::new(usize::MAX, usize::MAX);
    let mut max_coords = Coords::new(0, 0);
    let coords: Vec<_> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let c = Coords::new(x.parse().unwrap(), y.parse().unwrap());

            min_coords.x = min_coords.x.min(c.x);
            min_coords.y = min_coords.y.min(c.y);

            max_coords.x = max_coords.x.max(c.x);
            max_coords.y = max_coords.y.max(c.y);

            c
        })
        .collect();
    let coords: Vec<_> = coords
        .iter()
        .map(|c| Coords::new(c.x - min_coords.x, c.y - min_coords.y))
        .collect();
    println!("Done parsing coords");
    // println!("coords: {:?}", coords);

    let mut grid: Vec<Vec<Color>> = vec![];
    let mut grid_row = vec![];
    grid_row.resize(max_coords.x - 1, Color::None);
    grid.resize(max_coords.y, grid_row);
    println!("Done initialising grid");

    for i in 0..coords.len() {
        let current = coords[i];
        let previous = if i == 0 {
            coords.last().cloned().unwrap()
        } else {
            coords[i - 1]
        };

        if previous.y == current.y {
            // same row
            for i in 0..grid[current.y].len() {
                // println!("{} {:?}", i, (previous.x..current.x));
                if (previous.x + 1..current.x).contains(&i)
                    || (current.x + 1..previous.x).contains(&i)
                {
                    grid[current.y][i] = Color::Green
                }
            }
        }
        if previous.x == current.x {
            // same column
            for i in 0..grid.len() {
                if (previous.y + 1..current.y).contains(&i)
                    || (current.y + 1..previous.y).contains(&i)
                {
                    grid[i][current.x] = Color::Green
                }
            }
        }
        grid[current.y][current.x] = Color::Red;
    }
    println!("Done creating grid");

    grid.iter_mut().for_each(|row| {
        let Some(cstart) = row.iter().position(|&col| col != Color::None) else {
            return;
        };
        let Some(cend) = row.iter().rposition(|&col| col != Color::None) else {
            return;
        };

        row[cstart + 1..cend].fill(Color::Green);
    });
    println!("Done filling grid");

    // println!(
    //     "{}",
    //     grid.iter()
    //         .map(|row| row
    //             .iter()
    //             .map(|col| format!("{:?}", col))
    //             .collect::<String>())
    //         .collect::<Vec<_>>()
    //         .join("\n")
    // );

    // let mut rect_corners = vec![];
    let mut coord_pairs = vec![];
    for i in 0..coords.len() - 2 {
        for j in i + 2..coords.len() {
            coord_pairs.push((coords[i], coords[j]));
        }
    }
    println!("Done creating coord pairs");

    coord_pairs.sort_by(|a, b| a.0.rect(&a.1).cmp(&b.0.rect(&b.1)));
    coord_pairs.reverse();
    println!("Done sorting coord pairs");

    for coord_pair in coord_pairs {
        let (x1, x2) = if coord_pair.0.x < coord_pair.1.x {
            (coord_pair.0.x, coord_pair.1.x)
        } else {
            (coord_pair.1.x, coord_pair.0.x)
        };
        let (y1, y2) = if coord_pair.0.y < coord_pair.1.y {
            (coord_pair.0.y, coord_pair.1.y)
        } else {
            (coord_pair.1.y, coord_pair.0.y)
        };

        if grid[y1..y2]
            .par_iter()
            .any(|row| row[x1..x2].par_iter().any(|c| c == &Color::None))
        {
            println!("skipped: {:?}", coord_pair);
            continue;
        }

        println!(
            "max rect: {:?} {}",
            coord_pair,
            coord_pair.0.rect(&coord_pair.1)
        );
        return;
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn rect(&self, other: &Self) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}
impl Debug for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Color {
    Red,
    Green,
    None,
}
impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::None => ".",
                Color::Red => "\x1b[31mR\x1b[0m",
                Color::Green => "\x1b[32mG\x1b[0m",
            }
        )
    }
}
