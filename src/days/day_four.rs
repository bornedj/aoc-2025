#[derive(Debug, PartialEq, Clone)]
enum Tile {
    PaperRoll,
    Empty,
    RemovableRoll,
}

pub fn process_day_four_puzzle_two(s: &str) -> u32 {
    let mut board = transform_input_to_board(s);
    let mut count: u32 = 0;

    // do while, local count will be checked at the end of the loop
    loop {
        let mut local_count: u32 = 0;

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                let is_removable = check_cell(i, j, &board[i][j], &board);
                local_count += is_removable as u32;
                if is_removable {
                    board[i][j] = Tile::RemovableRoll;
                }
            }
        }

        board = board.iter_mut().map(|col| {
            col.iter_mut().map(|tile| {
                match tile {
                    Tile::RemovableRoll => Tile::Empty,
                    _ => tile.clone(),
                }
            }).collect()
        }).collect();

        count += local_count;
        if local_count == 0 {
            break;
        }
    }

    count
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
        } else if let Tile::RemovableRoll = board[col_index][row_index - 1] {
            count_paper_neighbors += 1;
        }

        if col_index > 0 {
            if let Tile::PaperRoll = board[col_index - 1][row_index - 1] {
                count_paper_neighbors += 1;
            } else if let Tile::RemovableRoll = board[col_index - 1][row_index - 1] {
                count_paper_neighbors += 1;
            }
        } 

        if col_index < board_height - 1 {
            if let Tile::PaperRoll = board[col_index + 1][row_index - 1] {
                count_paper_neighbors += 1;
            } else if let Tile::RemovableRoll = board[col_index + 1][row_index - 1] {
                count_paper_neighbors += 1;
            }
        }
    } 

    // right checks
    if row_index < board_width - 1 {
        if let Tile::PaperRoll = board[col_index][row_index + 1] {
            count_paper_neighbors += 1;
        } else if let Tile::RemovableRoll = board[col_index][row_index + 1] {
            count_paper_neighbors += 1;
        }

        if col_index > 0 {
            if let Tile::PaperRoll = board[col_index - 1][row_index + 1] {
                count_paper_neighbors += 1;
            } else if let Tile::RemovableRoll = board[col_index - 1][row_index + 1] {
                count_paper_neighbors += 1;
            }
        }

        if col_index < board_height - 1 {
            if let Tile::PaperRoll = board[col_index + 1][row_index + 1] {
                count_paper_neighbors += 1;
            } else if let Tile::RemovableRoll = board[col_index + 1][row_index + 1] {
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
        Tile::RemovableRoll => panic!("Should not be reachable when invoked")
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
    fn puzzle_two() {
        assert_eq!(43, process_day_four_puzzle_two(EXAMPLE_INPUT));
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
