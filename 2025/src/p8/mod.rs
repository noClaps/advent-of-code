use std::{collections::VecDeque, fmt::Debug};

pub fn p8() {
    let input = include_str!("./input.txt").trim();

    let junctions: Vec<_> = input
        .lines()
        .map(|line| line.splitn(3, ',').collect::<Vec<_>>())
        .map(|parts| {
            Junction::new(
                parts[0].parse().unwrap(),
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )
        })
        .collect();

    let mut connections = vec![];
    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            connections.push((junctions[i], junctions[j]));
        }
    }

    connections.sort_by(|a, b| a.0.distance(a.1).total_cmp(&b.0.distance(b.1)));
    // println!("connections count: {:?}", connections.len());
    // const ITERATIONS: usize = 1000;

    let mut connections = VecDeque::from(connections);
    let mut circuits: Vec<Vec<Junction>> = vec![];
    let mut last_connection = connections[0];
    'outer: while !circuits.iter().any(|c| c.len() == junctions.len()) {
        let connection = connections.pop_front().unwrap();
        last_connection = connection;
        // println!("circuits: {:?}", circuits);
        // println!("connection: {:?}", connection);
        for i in 0..circuits.len() {
            for j in i + 1..circuits.len() {
                if circuits[i].contains(&connection.0) && circuits[j].contains(&connection.1) {
                    let mut circ = circuits.remove(j);
                    circuits[i].append(&mut circ);
                    continue 'outer;
                }
                if circuits[i].contains(&connection.1) && circuits[j].contains(&connection.0) {
                    let mut circ = circuits.remove(j);
                    circuits[i].append(&mut circ);
                    continue 'outer;
                }
            }
        }

        for c in 0..circuits.len() {
            if circuits[c].contains(&connection.0) && circuits[c].contains(&connection.1) {
                // println!(
                //     "already connected: [{:?}, {:?}]",
                //     connection.0, connection.1
                // );
                continue 'outer;
            }
            if circuits[c].contains(&connection.0) && !circuits[c].contains(&connection.1) {
                // println!("adding {:?} to {:?}", connection.1, circuits[c]);
                circuits[c].push(connection.1);
                continue 'outer;
            }
            if !circuits[c].contains(&connection.0) && circuits[c].contains(&connection.1) {
                // println!("adding {:?} to {:?}", connection.0, circuits[c]);
                circuits[c].push(connection.0);
                continue 'outer;
            }
        }

        // println!("new circuit: {:?}", [connection.0, connection.1]);
        circuits.push(vec![connection.0, connection.1]);
    }
    // println!("circuits: {:?}", circuits);

    // let mut circuit_lens: Vec<_> = circuits.iter().map(|c| c.len()).collect();
    // circuit_lens.sort();
    // circuit_lens.reverse();
    // println!("circuit lens: {:?}", circuit_lens);

    // let prod = circuit_lens[0] * circuit_lens[1] * circuit_lens[2];
    // println!("answer: {}", prod);

    println!("answer: {:?}", last_connection.0.x * last_connection.1.x);
}

#[derive(Clone, Copy, PartialEq)]
struct Junction {
    x: isize,
    y: isize,
    z: isize,
}

impl Junction {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
    fn distance(&self, b: Self) -> f64 {
        let diff_x = self.x - b.x;
        let diff_y = self.y - b.y;
        let diff_z = self.z - b.z;

        ((diff_x.pow(2) + diff_y.pow(2) + diff_z.pow(2)) as f64).sqrt()
    }
}

impl Debug for Junction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
