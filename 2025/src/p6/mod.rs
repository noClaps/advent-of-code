#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

pub fn p6() {
    let input = include_str!("./input.txt").trim();
    let lines: Vec<_> = input.lines().collect();

    let operators = lines[lines.len() - 1];
    let mut operations = vec![];
    for c in operators.chars().rev() {
        match c {
            '+' => operations.push(Operation::Add),
            '*' => operations.push(Operation::Multiply),
            _ => (),
        }
    }

    let lines = &lines[0..lines.len() - 1];
    let mut col_lengths = vec![];
    for line in lines {
        let numbers: Vec<_> = line.split_whitespace().collect();
        for i in 0..numbers.len() {
            if col_lengths.len() > i {
                if col_lengths[i] < numbers[i].len() {
                    col_lengths[i] = numbers[i].len()
                }
            } else {
                col_lengths.push(numbers[i].len());
            }
        }
    }

    let mut groups: Vec<Vec<_>> = vec![];
    for line in lines {
        let mut num_str: Vec<String> = vec![];
        let mut i = 0;
        for col in &col_lengths {
            num_str.push(line[i..i + col].to_string());
            i += col + 1;
        }
        groups.push(num_str);
    }

    let mut group_nums = vec![];
    for i in (0..groups[0].len()).rev() {
        let mut transpose: Vec<Vec<char>> = vec![];
        for row in &groups {
            transpose.push(row[i].chars().rev().collect());
        }

        let mut new_transpose: Vec<usize> = vec![];
        for n in 0..transpose[0].len() {
            let mut num = String::new();
            for item in &transpose {
                num.push(item[n]);
            }
            new_transpose.push(num.trim().parse().unwrap());
        }
        group_nums.push(new_transpose);
    }

    let mut answers = vec![];
    for i in 0..group_nums.len() {
        let answer = group_nums[i]
            .clone()
            .into_iter()
            .reduce(|acc, e| match operations[i] {
                Operation::Add => acc + e,
                Operation::Multiply => acc * e,
            });
        answers.push(answer.unwrap());
    }

    println!("{}", answers.iter().sum::<usize>())
}
