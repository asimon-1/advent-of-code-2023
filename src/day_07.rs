use std::collections::HashMap;

enum HandType {
    FIVEOFAKIND = 0x600000,
    FOUROFAKIND = 0x500000,
    FULLHOUSE = 0x400000,
    THREEOFAKIND = 0x300000,
    TWOPAIR = 0x200000,
    ONEPAIR = 0x100000,
    HIGHCARD = 0x000000,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    JOKER = 0x1,
    TWO = 0x2,
    THREE = 0x3,
    FOUR = 0x4,
    FIVE = 0x5,
    SIX = 0x6,
    SEVEN = 0x7,
    EIGHT = 0x8,
    NINE = 0x9,
    TEN = 0xA,
    JACK = 0xB,
    QUEEN = 0xC,
    KING = 0xD,
    ACE = 0xE,
}

impl Card {
    fn from_char_part_a(c: char) -> Card {
        match c {
            '2' => Card::TWO,
            '3' => Card::THREE,
            '4' => Card::FOUR,
            '5' => Card::FIVE,
            '6' => Card::SIX,
            '7' => Card::SEVEN,
            '8' => Card::EIGHT,
            '9' => Card::NINE,
            'T' => Card::TEN,
            'J' => Card::JACK,
            'Q' => Card::QUEEN,
            'K' => Card::KING,
            'A' => Card::ACE,
            _ => panic!("BAD CHARACTER"),
        }
    }

    fn from_char_part_b(c: char) -> Card {
        match c {
            'J' => Card::JOKER,
            '2' => Card::TWO,
            '3' => Card::THREE,
            '4' => Card::FOUR,
            '5' => Card::FIVE,
            '6' => Card::SIX,
            '7' => Card::SEVEN,
            '8' => Card::EIGHT,
            '9' => Card::NINE,
            'T' => Card::TEN,
            'Q' => Card::QUEEN,
            'K' => Card::KING,
            'A' => Card::ACE,
            _ => panic!("BAD CHARACTER"),
        }
    }
}

struct Hand {
    cards: Vec<Card>,
    bid: u64,
    score: u64,
}

impl Hand {
    fn score_hand(&mut self) {
        let hand_type = self.get_hand_type();
        self.score = (hand_type as u64)
            + (self.cards[0] as u64) * 0x10000
            + (self.cards[1] as u64) * 0x01000
            + (self.cards[2] as u64) * 0x00100
            + (self.cards[3] as u64) * 0x00010
            + (self.cards[4] as u64) * 0x00001
    }

    fn get_hand_type(&self) -> HandType {
        let mut map: HashMap<Card, usize> = HashMap::new();
        for c in self.cards.iter() {
            *map.entry(*c).or_default() += 1;
        }

        let (_, jokers) = map.remove_entry(&Card::JOKER).unwrap_or((Card::JOKER, 0));

        let max_card = *map
            .iter()
            .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
            .unwrap_or((&Card::JOKER, &0))
            .0;
        let _ = map.entry(max_card).or_default();
        map.entry(max_card).and_modify(|e| *e += jokers);

        let max = *map.values().max().unwrap();
        let min = *map.values().min().unwrap();
        let unique = map.values().count();
        if max == 5 {
            HandType::FIVEOFAKIND
        } else if max == 4 {
            HandType::FOUROFAKIND
        } else if max == 3 && min == 2 {
            HandType::FULLHOUSE
        } else if max == 3 && min == 1 {
            HandType::THREEOFAKIND
        } else if max == 2 && unique == 3 {
            HandType::TWOPAIR
        } else if max == 2 && unique == 4 {
            HandType::ONEPAIR
        } else {
            HandType::HIGHCARD
        }
    }
}

fn parse_input_part_a(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let split = line.split_once(" ").unwrap();
            let mut hand = Hand {
                cards: split.0.chars().map(|c| Card::from_char_part_a(c)).collect(),
                bid: split.1.parse().unwrap(),
                score: 0,
            };
            hand.score_hand();
            hand
        })
        .collect()
}

fn parse_input_part_b(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let split = line.split_once(" ").unwrap();
            let mut hand = Hand {
                cards: split.0.chars().map(|c| Card::from_char_part_b(c)).collect(),
                bid: split.1.parse().unwrap(),
                score: 0,
            };
            hand.score_hand();
            hand
        })
        .collect()
}

fn part_a(input: &str) -> u64 {
    let mut hands = parse_input_part_a(input);
    hands.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| ((index + 1_usize) as u64) * hand.bid)
        .sum()
}

fn part_b(input: &str) -> u64 {
    let mut hands = parse_input_part_b(input);
    hands.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| ((index + 1_usize) as u64) * hand.bid)
        .sum()
}

pub use crate::boilerplate;

boilerplate!(7, 6440, 5905, u64);
