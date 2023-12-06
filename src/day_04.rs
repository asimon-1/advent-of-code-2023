use std::collections::VecDeque;

#[derive(Clone)]
struct Card {
    num: u32,
    winning_numbers: Vec<u32>,
    draws: Vec<u32>,
}

impl Card {
    fn from_line(line: &str) -> Card {
        let splits: Vec<&str> = line.split(is_delimiter).collect();
        let mut num = 0;
        let mut winning_numbers = Vec::new();
        let mut draws = Vec::new();
        for (index, split) in splits.iter().enumerate() {
            match index {
                0 => {
                    num = split.split_whitespace().last().unwrap().parse().unwrap();
                }
                1 => {
                    winning_numbers = split
                        .split_whitespace()
                        .map(|m| m.parse().unwrap())
                        .collect();
                }
                2 => {
                    draws = split
                        .split_whitespace()
                        .map(|m| m.parse().unwrap())
                        .collect();
                }
                _ => panic!("Shouldn't be more than two delimiters!"),
            }
        }
        Card {
            num,
            winning_numbers,
            draws,
        }
    }
    fn num_matches(&self) -> u32 {
        let mut matches: u32 = 0;
        for winning_num in &self.winning_numbers {
            for draw in &self.draws {
                if draw == winning_num {
                    matches += 1;
                }
            }
        }
        matches
    }

    fn score(&self) -> u32 {
        let score_multiplier = self.num_matches();
        if score_multiplier > 0 {
            2_u32.pow(score_multiplier - 1)
        } else {
            0
        }
    }
}

fn is_delimiter(c: char) -> bool {
    [':', '|'].contains(&c)
}

fn part_a(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Card::from_line(line))
        .map(|card| card.score())
        .sum()
}

fn part_b(input: &str) -> u32 {
    let cards: VecDeque<Card> = input.lines().map(|line| Card::from_line(line)).collect();
    let mut cards_to_process: VecDeque<Card> = cards.clone();
    let mut num_cards = 0;
    while !cards_to_process.is_empty() {
        let card = cards_to_process.pop_front().unwrap();
        let m = card.num_matches();
        if m > 0 {
            for ind in card.num..(card.num + m) {
                cards_to_process.push_back(cards[ind as usize].clone());
            }
        }
        num_cards += 1;
    }
    num_cards
}

pub use crate::boilerplate;

boilerplate!(4, 13, 30, u32);
