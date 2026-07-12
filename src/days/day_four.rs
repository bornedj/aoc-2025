#[derive(Debug, PartialEq)]
enum Tile {
    PaperRoll,
    Empty,
}

pub fn process_day_four_prob_one(s: &str) -> u32 {
    let board = transform_input_to_board(s);
    let mut count: u32 = 0;
    board.iter().enumerate().for_each(|(i, col)| {
        col.iter()
            .enumerate()
            .for_each(|(j, tile)| count += (check_cell(i, j, tile, &board)) as u32);
    });
    count
}

fn count_neighbors(col_index: usize, row_index: usize, board: &Vec<Vec<Tile>>) -> bool {
    let board_height = board.len();
    let board_width = board[0].len();

    let mut count_paper_neighbors = 0;

    // left checks
    if row_index > 0 {
        if let Tile::PaperRoll = board[col_index][row_index - 1] {
            count_paper_neighbors += 1;
        }

        if col_index > 0 {
            if let Tile::PaperRoll = board[col_index - 1][row_index - 1] {
                count_paper_neighbors += 1;
            }
        } 

        if col_index < board_height - 1 {
            if let Tile::PaperRoll = board[col_index + 1][row_index - 1] {
                count_paper_neighbors += 1;
            }
        }
    } 

    // right checks
    if row_index < board_width - 1 {
        if let Tile::PaperRoll = board[col_index][row_index + 1] {
            count_paper_neighbors += 1;
        }

        if col_index > 0 {
            if let Tile::PaperRoll = board[col_index - 1][row_index + 1] {
                count_paper_neighbors += 1;
            }
        }

        if col_index < board_height - 1 {
            if let Tile::PaperRoll = board[col_index + 1][row_index + 1] {
                count_paper_neighbors += 1;
            }
        }
    }

    // up and down checks
    if col_index > 0 {
        if let Tile::PaperRoll = board[col_index - 1][row_index] {
            count_paper_neighbors += 1;
        }
    }

    if col_index < board_height - 1 {
        if let Tile::PaperRoll = board[col_index + 1][row_index] {
            count_paper_neighbors += 1;
        }
    }

    count_paper_neighbors < 4
}

fn check_cell(col_index: usize, row_index: usize, tile: &Tile, board: &Vec<Vec<Tile>>) -> bool {
    match &tile {
        Tile::Empty => false,
        Tile::PaperRoll => {
            count_neighbors(col_index, row_index, board)
        },
    }
}


fn transform_line_to_tile(s: &str) -> Vec<Tile> {
    s.trim().chars().map(char_to_tile).collect()
}

fn char_to_tile(c: char) -> Tile {
    match c {
        '@' => return Tile::PaperRoll,
        '.' => return Tile::Empty,
        _ => panic!("Should only be the above cases"),
    }
}

fn transform_input_to_board(s: &str) -> Vec<Vec<Tile>> {
    s.lines().map(transform_line_to_tile).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn example_integration() {
        assert_eq!(13, process_day_four_prob_one(EXAMPLE_INPUT));
    }

    #[test]
    fn can_tranform_line() {
        let result = vec![
            Tile::Empty,
            Tile::Empty,
            Tile::PaperRoll,
            Tile::PaperRoll,
            Tile::Empty,
            Tile::PaperRoll,
            Tile::PaperRoll,
            Tile::PaperRoll,
            Tile::PaperRoll,
            Tile::Empty,
        ];
        let line = EXAMPLE_INPUT.lines().next().unwrap();
        assert_eq!(result, transform_line_to_tile(line));
    }
}
