fn part_a(input: &str) -> u32 {
    let mask_false = input
        .lines()
        .map(|line| line.chars().map(|_| false).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let rows: usize = mask_false.len();
    let cols: usize = mask_false[0].len();

    let mask_numeric = input
        .lines()
        .map(|line| line.chars().map(|c| c.is_numeric()).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    // Build mask of where the symbols are
    let mask_symbol = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| !x.is_numeric() && x != '.')
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    // Build mask of any cells adjacent to symbols
    let mut mask_symbol_adjacent = mask_false.clone();

    for row in 0..rows {
        for col in 0..cols {
            let row_before = row.saturating_sub(1);
            let col_before = col.saturating_sub(1);
            let row_after = (rows - 1).min(row + 1);
            let col_after = (cols - 1).min(col + 1);

            if mask_symbol[row][col] {
                mask_symbol_adjacent[row_before][col_before] = true;
                mask_symbol_adjacent[row_before][col] = true;
                mask_symbol_adjacent[row_before][col_after] = true;
                mask_symbol_adjacent[row][col_before] = true;
                mask_symbol_adjacent[row][col] = true;
                mask_symbol_adjacent[row][col_after] = true;
                mask_symbol_adjacent[row_after][col_before] = true;
                mask_symbol_adjacent[row_after][col] = true;
                mask_symbol_adjacent[row_after][col_after] = true;
            }
        }
    }

    // Build mask of any numeric cells which are adjacent to symbols

    let mut mask_numeric_symbol_adjacent = mask_false.clone();

    for row in 0..rows {
        for col in 0..cols {
            mask_numeric_symbol_adjacent[row][col] =
                mask_numeric[row][col] && mask_symbol_adjacent[row][col];
        }
    }

    // Expand mask of numeric cells which are adjacent to symbols to include any additional numeric adjacent cells

    for _ in 0..3 {
        // Maximum length of a number is 3 from inspection of the input
        let mut temp = mask_numeric_symbol_adjacent.clone();
        for row in 0..rows {
            for col in 0..cols {
                let col_before = col.saturating_sub(1);
                let col_after = (mask_symbol[0].len() - 1).min(col + 1);

                if mask_numeric_symbol_adjacent[row][col] && mask_numeric[row][col_before] {
                    temp[row][col_before] = true;
                }
                if mask_numeric_symbol_adjacent[row][col] && mask_numeric[row][col_after] {
                    temp[row][col_after] = true;
                }
            }
        }
        mask_numeric_symbol_adjacent = temp.clone();
    }

    // Filter the input with our mask
    let mut input_chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for row in 0..rows {
        for col in 0..cols {
            if !mask_numeric_symbol_adjacent[row][col] {
                input_chars[row][col] = ' ';
            }
        }
    }

    // Parse the filtered input and sum
    input_chars
        .iter()
        .map(|line| line.iter().collect::<String>())
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[allow(unused_variables)]
fn part_b(input: &str) -> u32 {
    0
}

pub use crate::boilerplate;

boilerplate!(3, 4361, 0, u32);
