fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn part_a(input: &str) -> i32 {
    let histories = parse_input(input);
    histories
        .iter()
        .map(|history| {
            let mut sequences: Vec<Vec<i32>> = Vec::new();
            sequences.push(history.clone());
            while !sequences.last().unwrap().iter().all(|x| *x == 0) {
                let sequence = sequences
                    .last()
                    .unwrap()
                    .iter()
                    .map_windows(|&[a, b]| b - a)
                    .collect();
                sequences.push(sequence);
            }
            sequences
                .iter()
                .map(|sequence| sequence.last().unwrap())
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn part_b(input: &str) -> i32 {
    let histories = parse_input(input);
    histories
        .iter()
        .map(|history| {
            let mut sequences: Vec<Vec<i32>> = Vec::new();
            sequences.push(history.clone());
            while !sequences.last().unwrap().iter().all(|x| *x == 0) {
                let sequence = sequences
                    .last()
                    .unwrap()
                    .iter()
                    .map_windows(|&[a, b]| b - a)
                    .collect();
                sequences.push(sequence);
            }
            sequences
                .iter()
                .enumerate()
                .map(|(index, sequence)| (-1_i32).pow(index as u32) * sequence.first().unwrap())
                .sum::<i32>()
        })
        .sum::<i32>()
}

pub use crate::boilerplate;

boilerplate!(9, 114, 2, i32);
