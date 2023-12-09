#![feature(iter_map_windows)]
use std::fs;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

macro_rules! run_all {
    ($($l:ident),+ $(,)?) => {
        {
            $(
                println!("{}a: {}", stringify!($l), $l::run_part_a());
                println!("{}b: {}\n", stringify!($l), $l::run_part_b());
            )+
        }
    }
}

#[macro_export]
macro_rules! boilerplate {
    ($day:literal, $test_part_a:literal, $test_part_b: literal, $ret_type: ty) => {
        use crate::get_input;
        #[allow(dead_code)]
        pub fn run_part_a() -> $ret_type {
            let input = get_input($day, false, true).unwrap();
            let input = input.as_str();
            part_a(input)
        }

        #[allow(dead_code)]
        pub fn run_part_b() -> $ret_type {
            let input = get_input($day, false, false).unwrap();
            let input = input.as_str();
            part_b(input)
        }

        #[test]
        fn part_a_test() {
            let input = get_input($day, true, true).unwrap();
            let input = input.as_str();
            assert_eq!(part_a(input), $test_part_a);
        }

        #[test]
        fn part_b_test() {
            let input = get_input($day, true, false).unwrap();
            let input = input.as_str();
            assert_eq!(part_b(input), $test_part_b);
        }
    };
}

fn get_input(day: u8, test: bool, part_a: bool) -> Result<String, std::io::Error> {
    let fname = if part_a {
        format!("{}a.txt", day)
    } else {
        format!("{}b.txt", day)
    };
    let path = if test {
        format!("./input_test/{}", fname)
    } else {
        format!("./input/{}", fname)
    };
    fs::read_to_string(path)
}

fn main() {
    run_all!(
        // day_01,
        // day_02,
        // day_03,
        // day_04, // Day 4 part B takes a long time
        // day_05, // Day 5 part B take a long time
        // day_06,
        // day_07,
        // day_08,
        day_09,
    );
}
