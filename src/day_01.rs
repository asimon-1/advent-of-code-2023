fn part_a(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().filter(|char| char.is_numeric()))
        .map(|mut line| {
            let first = line.next().unwrap();
            let last = line.last().unwrap_or(first);
            [first, last].iter().collect::<String>()
        })
        .map(|num| num.parse::<u32>().unwrap_or(0))
        .sum()
}

fn part_b(input: &str) -> u32 {
    input
        .lines()
        .map(|line| find_lower(line) * 10 + find_upper(line))
        .sum()
}

fn find_lower(line: &str) -> u32 {
    let ind = [
        line.find("1"),
        line.find("2"),
        line.find("3"),
        line.find("4"),
        line.find("5"),
        line.find("6"),
        line.find("7"),
        line.find("8"),
        line.find("9"),
        line.find("one"),
        line.find("two"),
        line.find("three"),
        line.find("four"),
        line.find("five"),
        line.find("six"),
        line.find("seven"),
        line.find("eight"),
        line.find("nine"),
    ]
    .iter()
    .filter_map(|x| *x)
    .min()
    .unwrap();
    let ret: u32 = match line.get(ind..=ind).unwrap() {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "o" => 1,
        "t" => match line.get(ind + 1..=ind + 1).unwrap() {
            // Both "two" and "three" start with "t" so we have to look at
            // the next character to determine which one we have
            "w" => 2,
            "h" => 3,
            _ => unreachable!(), // we already confirmed that its one of these two
        },
        "f" => match line.get(ind + 1..=ind + 1).unwrap() {
            // Both "four" and "five" start with "f" so we have to look at
            // the next character to determine which one we have
            "o" => 4,
            "i" => 5,
            _ => unreachable!(), // we already confirmed that its one of these two
        },
        "s" => match line.get(ind + 1..=ind + 1).unwrap() {
            // Both "six" and "seven" start with "t" so we have to look at
            // the next character to determine which one we have
            "i" => 6,
            "e" => 7,
            _ => unreachable!(), // we already confirmed that its one of these two
        },
        "e" => 8,
        "n" => 9,
        _ => unreachable!(), // we already confirmed that its one of these options
    };
    ret
}

fn find_upper(line: &str) -> u32 {
    let ind = [
        line.rfind("1"),
        line.rfind("2"),
        line.rfind("3"),
        line.rfind("4"),
        line.rfind("5"),
        line.rfind("6"),
        line.rfind("7"),
        line.rfind("8"),
        line.rfind("9"),
        line.rfind("one"),
        line.rfind("two"),
        line.rfind("three"),
        line.rfind("four"),
        line.rfind("five"),
        line.rfind("six"),
        line.rfind("seven"),
        line.rfind("eight"),
        line.rfind("nine"),
    ]
    .iter()
    .filter_map(|x| *x)
    .max()
    .unwrap();
    let ret: u32 = match line.get(ind..=ind).unwrap() {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "o" => 1,
        "t" => match line.get(ind + 1..=ind + 1).unwrap() {
            // Both "two" and "three" start with "t" so we have to look at
            // the next character to determine which one we have
            "w" => 2,
            "h" => 3,
            _ => unreachable!(), // we already confirmed that its one of these two
        },
        "f" => match line.get(ind + 1..=ind + 1).unwrap() {
            // Both "four" and "five" start with "f" so we have to look at
            // the next character to determine which one we have
            "o" => 4,
            "i" => 5,
            _ => unreachable!(), // we already confirmed that its one of these two
        },
        "s" => match line.get(ind + 1..=ind + 1).unwrap() {
            // Both "six" and "seven" start with "t" so we have to look at
            // the next character to determine which one we have
            "i" => 6,
            "e" => 7,
            _ => unreachable!(), // we already confirmed that its one of these two
        },
        "e" => 8,
        "n" => 9,
        _ => unreachable!(), // we already confirmed that its one of these options
    };
    ret
}

pub use crate::boilerplate;
boilerplate!(1, 142, 281, u32);
