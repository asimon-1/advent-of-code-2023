use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PipeSegment {
    pos: (i32, i32),
    to: (i32, i32),
    from: (i32, i32),
}

fn parse_input(input: &str) -> ((i32, i32), HashMap<(i32, i32), PipeSegment>) {
    let mut starting_pos = (0, 0);
    let mut map = HashMap::new();
    input.lines().enumerate().for_each(|(row_usize, line)| {
        let row = row_usize as i32;
        line.chars()
            .enumerate()
            .filter_map(|(col_usize, c)| {
                let col = col_usize as i32;
                match c {
                    '.' => None,
                    '|' => Some(PipeSegment {
                        pos: (row, col),
                        to: (row - 1, col),
                        from: (row + 1, col),
                    }),
                    '-' => Some(PipeSegment {
                        pos: (row, col),
                        to: (row, col - 1),
                        from: (row, col + 1),
                    }),
                    'L' => Some(PipeSegment {
                        pos: (row, col),
                        to: (row, col + 1),
                        from: (row - 1, col),
                    }),
                    'J' => Some(PipeSegment {
                        pos: (row, col),
                        to: (row, col - 1),
                        from: (row - 1, col),
                    }),
                    '7' => Some(PipeSegment {
                        pos: (row, col),
                        to: (row, col - 1),
                        from: (row + 1, col),
                    }),
                    'F' => Some(PipeSegment {
                        pos: (row, col),
                        to: (row, col + 1),
                        from: (row + 1, col),
                    }),
                    'S' => {
                        // We don't know where this connects to, so say it connects to itself
                        // Might regret this
                        starting_pos = (row, col);
                        Some(PipeSegment {
                            pos: (row, col),
                            to: (row, col),
                            from: (row, col),
                        })
                    }
                    _ => None,
                }
            })
            .for_each(|pipe| {
                map.insert(pipe.pos, pipe);
            })
    });
    (starting_pos, map)
}

fn find_loop(
    starting_pos: (i32, i32),
    mut map: HashMap<(i32, i32), PipeSegment>,
) -> Vec<(i32, i32)> {
    let mut loop_vals = vec![starting_pos.clone()];
    map.remove(&starting_pos);
    let starting_segments: Vec<PipeSegment> = map
        .values()
        .filter(|pipe| (pipe.from == starting_pos || pipe.to == starting_pos))
        .copied()
        .collect();
    let mut current_segment = map.remove(&starting_segments[0].pos);
    while current_segment.is_some() {
        let pipe = current_segment.unwrap();
        loop_vals.push(pipe.pos);
        let to = map.remove(&pipe.to);
        let from = map.remove(&pipe.from);
        if to.is_none() && from.is_none() {
            // Nowhere to go, we are at the end of the loop
            break;
        } else if to.is_some() {
            current_segment = to;
        } else if from.is_some() {
            current_segment = from;
        } else {
            panic!("Both from and to are some!");
        }
    }
    loop_vals
}

fn part_a(input: &str) -> i32 {
    let (starting_pos, map) = parse_input(input);
    let loop_vals = find_loop(starting_pos, map);
    ((loop_vals.len() + 1) / 2) as i32
}

fn part_b(input: &str) -> i32 {
    let (starting_pos, map) = parse_input(input);
    let loop_vals = find_loop(starting_pos, map.clone());
    let mut in_out: bool;
    let mut count = 0;
    for (row_ind, row) in input.lines().enumerate() {
        in_out = false;
        for (col_ind, character) in row.chars().enumerate() {
            if loop_vals.contains(&(row_ind as i32, col_ind as i32)) {
                if ['|', '7', 'F', 'S'].contains(&character) {
                    in_out = !in_out;
                }
            } else if in_out {
                count += 1;
            }
        }
    }

    count
}

pub use crate::boilerplate;

boilerplate!(10, 8, 10, i32);
