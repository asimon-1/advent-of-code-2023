use fancy_regex::Regex;

fn part_a(input: &str) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    input
        .lines()
        .map(|l| parse_game(l))
        .filter(|l| l[1] <= MAX_RED && l[2] <= MAX_GREEN && l[3] <= MAX_BLUE)
        .map(|l| l[0])
        .sum()
}

fn parse_game(game_line: &str) -> [u32; 4] {
    // Returns [Game #, max(Red), max(Green), max(Blue)]
    let re_game = Regex::new(r"(?<=Game )\d+").unwrap();
    let re_red = Regex::new(r"\d+(?= red)").unwrap();
    let re_green = Regex::new(r"\d+(?= green)").unwrap();
    let re_blue = Regex::new(r"\d+(?= blue)").unwrap();
    let game = re_game
        .find(game_line)
        .unwrap()
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();
    let red = re_red
        .find_iter(game_line)
        .map(|m| m.unwrap().as_str().parse::<u32>().unwrap())
        .max()
        .unwrap_or(0);
    let green = re_green
        .find_iter(game_line)
        .map(|m| m.unwrap().as_str().parse::<u32>().unwrap())
        .max()
        .unwrap_or(0);
    let blue = re_blue
        .find_iter(game_line)
        .map(|m| m.unwrap().as_str().parse::<u32>().unwrap())
        .max()
        .unwrap_or(0);
    [game, red, green, blue]
}

fn part_b(input: &str) -> u32 {
    input
        .lines()
        .map(|l| parse_game(l))
        .map(|l| l[1] * l[2] * l[3])
        .sum()
}

pub use crate::boilerplate;
boilerplate!(2, 8, 2286, u32);
