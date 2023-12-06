struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn get_ways_to_win(&self) -> u64 {
        (0..self.time)
            .map(|charge_time| self.does_win(charge_time))
            .filter(|x| *x)
            .count() as u64
    }

    fn does_win(&self, charge_time: u64) -> bool {
        get_distance_traveled(charge_time, self.time) > self.distance
    }
}

fn get_distance_traveled(charge_time: u64, time_limit: u64) -> u64 {
    charge_time * (time_limit - charge_time)
}

#[test]
fn test_get_distance_traveled() {
    assert_eq!(get_distance_traveled(0, 7), 0);
    assert_eq!(get_distance_traveled(1, 7), 6);
    assert_eq!(get_distance_traveled(2, 7), 10);
    assert_eq!(get_distance_traveled(3, 7), 12);
    assert_eq!(get_distance_traveled(4, 7), 12);
    assert_eq!(get_distance_traveled(5, 7), 10);
    assert_eq!(get_distance_traveled(6, 7), 6);
    assert_eq!(get_distance_traveled(7, 7), 0);
}

fn parse_input_a(input: &str) -> Vec<Race> {
    let input_filtered = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|substring| substring.parse::<u64>().ok())
        })
        .map(|line| line.collect::<Vec<u64>>())
        .collect::<Vec<Vec<u64>>>();
    let num_races = input_filtered[0].len();
    let mut ret = Vec::new();
    for i in 0..num_races {
        ret.push(Race {
            time: input_filtered[0][i],
            distance: input_filtered[1][i],
        })
    }
    ret
}

fn parse_input_b(input: &str) -> Race {
    let input_filtered = input
        .lines()
        .map(|line| {
            line.replace("Time:", "")
                .replace("Distance:", "")
                .replace(" ", "")
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>();
    Race {
        time: input_filtered[0],
        distance: input_filtered[1],
    }
}

fn part_a(input: &str) -> u64 {
    let races = parse_input_a(input);
    races.iter().map(|race| race.get_ways_to_win()).product()
}

fn part_b(input: &str) -> u64 {
    let race = parse_input_b(input);
    race.get_ways_to_win()
}

pub use crate::boilerplate;

boilerplate!(6, 288, 71503, u64);
