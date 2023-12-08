use num::integer::lcm;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
enum LeftRight {
    Left,
    Right,
}

impl LeftRight {
    fn from_char(c: char) -> LeftRight {
        match c {
            'L' => LeftRight::Left,
            'R' => LeftRight::Right,
            _ => panic!("Bad character!"),
        }
    }
}

fn parse_input(input: &str) -> (Vec<LeftRight>, HashMap<&str, (&str, &str)>) {
    let split: Vec<&str> = input.split("\r\n\r\n").collect();
    let directions: Vec<LeftRight> = split[0].chars().map(|c| LeftRight::from_char(c)).collect();
    let mut map = HashMap::new();
    for line in split[1].lines() {
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        map.insert(key, (left, right));
    }
    (directions, map)
}

fn part_a(input: &str) -> u64 {
    let (directions, map) = parse_input(input);
    let mut location = "AAA";
    let mut steps = 0;
    for dir in directions.iter().cycle() {
        steps += 1;
        let options = map.get(&location).unwrap();
        location = if *dir == LeftRight::Left {
            options.0
        } else {
            options.1
        };
        if location == "ZZZ" {
            break;
        }
    }
    steps
}

fn part_b(input: &str) -> u64 {
    let (directions, map) = parse_input(input);
    let locations: Vec<&str> = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| *key)
        .collect();
    locations
        .iter()
        .map(|location| {
            let mut current_location = *location;
            let mut steps: u64 = 0;
            for dir in directions.iter().cycle() {
                steps += 1;
                let options = map.get(current_location).unwrap();
                current_location = if *dir == LeftRight::Left {
                    options.0
                } else {
                    options.1
                };
                if current_location.ends_with('Z') {
                    break;
                }
            }
            steps
        })
        .reduce(|acc, e| lcm(acc, e))
        .unwrap()
}

pub use crate::boilerplate;

boilerplate!(8, 2, 6, u64);
