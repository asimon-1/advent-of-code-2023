struct Number {
    value: u32,
    coords: Vec<(usize, usize)>,
}

impl Number {
    fn is_adjacent(&self, coords: (usize, usize)) -> bool {
        self.coords.iter().any(|digit_coords| {
            digit_coords.0.abs_diff(coords.0) <= 1 && digit_coords.1.abs_diff(coords.1) <= 1
        })
    }
}

struct Symbol {
    char: char,
    coords: (usize, usize),
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let input_transformed: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let last_col_ind = input_transformed[0].len() - 1;
    for (row_ind, row) in input_transformed.iter().enumerate() {
        let mut num_str = String::new();
        let mut num_coords: Vec<(usize, usize)> = Vec::new();
        for (col_ind, character) in row.iter().enumerate() {
            if character.is_numeric() {
                num_str.push(*character);
                num_coords.push((row_ind, col_ind));
            } else {
                if *character != '.' {
                    // Symbol, save it to the list
                    symbols.push(Symbol {
                        char: *character,
                        coords: (row_ind, col_ind),
                    });
                }
            }
            if !character.is_numeric() || col_ind == last_col_ind {
                // We've reached a potential end to a number
                if num_str.len() == 0 {
                    // No number already in the queue
                    continue;
                } else {
                    // We just finished scanning a number, save it to our list
                    let number = Number {
                        value: num_str.parse().unwrap(),
                        coords: num_coords,
                    };
                    numbers.push(number);
                    num_str = String::new();
                    num_coords = Vec::new();
                }
            }
        }
    }
    (numbers, symbols)
}

fn part_a(input: &str) -> u32 {
    let (numbers, symbols) = parse_input(input);
    dbg!(numbers.len());
    dbg!(symbols.len());
    numbers
        .iter()
        .filter(|number| {
            symbols
                .iter()
                .any(|symbol| number.is_adjacent(symbol.coords))
        })
        .map(|number| number.value)
        .sum()
}

#[allow(unused_variables)]
fn part_b(input: &str) -> u32 {
    let (numbers, symbols) = parse_input(input);
    symbols
        .iter()
        .filter(|symbol| symbol.char == '*')
        .map(|gear| {
            numbers
                .iter()
                .filter(|number| number.is_adjacent(gear.coords))
                .collect::<Vec<&Number>>()
        })
        .filter(|adjacent_numbers| adjacent_numbers.len() == 2)
        .map(|adjacent_numbers| adjacent_numbers[0].value * adjacent_numbers[1].value)
        .sum()
}

pub use crate::boilerplate;

boilerplate!(3, 4361, 467835, u32);
