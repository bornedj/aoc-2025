use core::panic;

#[derive(Debug)]
enum Token {
    Start,
    Empty,
    Beam,
    Splitter
}



pub fn puzzle_one(input: &str) -> u64 {
    let board = parse_board(input);
    todo!();
}

fn parse_board(input: &str) -> Vec<Vec<Token>> {
    let mut beam_indices: Vec<usize> = Vec::new();
    let mut parsed_lines: Vec<Vec<Token>> = Vec::new();
    for line in input.lines() {
        let parsed_line: Vec<Token> = line.chars().enumerate().map(|(i, c)| {
            match c {
                'S' => {
                    beam_indices.push(i);
                    Token::Start
                },
                '.' => Token::Empty,
                '^' => Token::Splitter,
                _ => panic!("unexpected char"),

            }
        }).collect();
        parsed_lines.push(parsed_line);
    };

    println!("beam_indices: {:?}",beam_indices);
    println!("{parsed_lines:?}");
    todo!()
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
