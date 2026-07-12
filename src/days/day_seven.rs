use core::{fmt, panic};

#[derive(Debug)]
enum Token {
    Start,
    Empty,
    Beam,
    Splitter
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
        let parsed_line: Vec<Token> = line.chars().enumerate().map(|(i, c)| {
            match c {
                'S' => {
                    beam_indices.push(i);
                    Token::Start
                },
                '.' => {
                    if beam_indices.contains(&i) {
                        return Token::Beam
                    }
                    Token::Empty
                },
                '^' => {
                    if beam_indices.contains(&i) {
                        total_splits += 1;
                    }
                    beam_indices = beam_indices.clone().into_iter().filter(|&index| index != i).collect();

                    if i > 0 {
                        beam_indices.push(i-1);
                    }
                    if i < line.len() - 1{
                        beam_indices.push(i+1);
                    }

                    Token::Splitter
                },
                _ => panic!("unexpected char"),

            }
        }).collect();
        parsed_lines.push(parsed_line);
    };

    let grid = Grid(parsed_lines);
    println!("beam_indices: {:?}",beam_indices);
    println!("{grid}");

    total_splits
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
}
