use core::{fmt, panic};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Token {
    Start,
    Empty,
    Beam,
    Splitter,
}

struct Grid(Vec<Vec<Token>>);

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for token in row {
                write!(f, "{token:?}, ")?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}

pub fn puzzle_one(input: &str) -> u32 {
    let mut beam_indices: Vec<usize> = Vec::new();
    let mut parsed_lines: Vec<Vec<Token>> = Vec::new();
    let mut total_splits: u32 = 0;

    for line in input.lines() {
        let parsed_line: Vec<Token> = line
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                'S' => {
                    beam_indices.push(i);
                    Token::Start
                }
                '.' => {
                    if beam_indices.contains(&i) {
                        return Token::Beam;
                    }
                    Token::Empty
                }
                '^' => {
                    if beam_indices.contains(&i) {
                        total_splits += 1;
                    }
                    beam_indices = beam_indices
                        .clone()
                        .into_iter()
                        .filter(|&index| index != i)
                        .collect();

                    if i > 0 {
                        beam_indices.push(i - 1);
                    }
                    if i < line.len() - 1 {
                        beam_indices.push(i + 1);
                    }

                    Token::Splitter
                }
                _ => panic!("unexpected char"),
            })
            .collect();
        parsed_lines.push(parsed_line);
    }

    total_splits
}

pub fn puzzle_two(input: &str) -> u64  {
    let grid = map_input_to_grid(input);
    let mut memoization: HashMap<(usize, usize), u64> = HashMap::new();
    let starting_col = grid.0[0].iter().position(|token| token == &Token::Start).expect("Must be starter in first row");

    compute_timelines(&grid, 1, starting_col, &mut memoization)
}

fn map_input_to_grid(input: &str) -> Grid {
    let mut parsed_lines: Vec<Vec<Token>> = Vec::new();
    for line in input.lines() {
        let parsed_line: Vec<Token> = line
            .chars()
            .map(|c| match c {
                'S' => {
                    Token::Start
                }
                '.' => {
                    Token::Empty
                }
                '^' => {
                    Token::Splitter
                }
                _ => panic!("unexpected char"),
            })
            .collect();
        parsed_lines.push(parsed_line);
    }
    Grid(parsed_lines)
}

fn compute_timelines(grid: &Grid, row: usize, col: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some(&value) = memo.get(&(row, col)) {
        return value;
    }

    // beam leaving bottom
    if row + 1 >= grid.0.len() {
        return 1;
    }

    let result = match grid.0[row + 1][col] {
        // going down
        Token::Empty => compute_timelines(grid, row+1, col, memo),
        // split to either side
        Token::Splitter => {
            compute_timelines(grid, row+1, col-1, memo) + compute_timelines(grid, row + 1, col + 1, memo)
        },
        _ => 1, // off the grid
    };

    memo.insert((row, col), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn example_input() {
        assert_eq!(21, puzzle_one(EXAMPLE_INPUT))
    }

    #[test]
    fn test_puzzle_two() {
        assert_eq!(40, puzzle_two(EXAMPLE_INPUT));
    }
}
