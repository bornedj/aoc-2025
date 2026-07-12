use core::{panic, str};
use std::ops::Range;

#[derive(Debug, PartialEq)]
enum Token {
    Number(u64),
    Operator(char),
}

pub fn process_puzzle_one(input: &str) -> u64 {
    let mut home_work_sheet = parse_input(input);
    let operators = home_work_sheet
        .pop()
        .expect("Expected operators row at the end");

    iterate_operators(&operators, &home_work_sheet, add_col, multiply_col)
}

pub fn process_puzzle_two(input: &str) -> u64 {
    let (ops, number_groups) = vertical_parse(input);

    let mut total = 0;
    let mut iter_numbs = number_groups.iter();
    for op in ops {
        match op {
            Token::Operator(o) => match o {
                '*' => total += iter_numbs.next().unwrap().iter().product::<u64>(),
                '+' => total += iter_numbs.next().unwrap().iter().sum::<u64>(),
                _ => panic!("not reachable")
            },
            _ => panic!("not reachable")
        }
    }
    total
}

fn iterate_operators(
    operators: &Vec<Token>,
    sheet: &Vec<Vec<Token>>,
    add_fn: fn(&Vec<&Token>) -> u64,
    multiply_fn: fn(&Vec<&Token>) -> u64,
) -> u64 {
    let mut total = 0;

    for (i, operator) in operators.iter().enumerate() {
        match operator {
            Token::Number(_) => panic!("Expected operator, found number"),
            Token::Operator(op) => {
                let col = extract_col(sheet, i);
                match op {
                    '*' => total += multiply_fn(&col),
                    '+' => total += add_fn(&col),
                    _ => panic!("Unknown operator: {}", op),
                }
            }
        }
    }
    total
}

fn multiply_col(col: &Vec<&Token>) -> u64 {
    col.iter().fold(1, |acc, curr| match curr {
        Token::Operator(_) => panic!("Should only be numbers left in the matrix"),
        Token::Number(num) => acc * num,
    })
}

fn add_col(col: &Vec<&Token>) -> u64 {
    col.iter().fold(0, |acc, curr| match curr {
        Token::Operator(_) => panic!("Should only be numbers left in the matrix"),
        Token::Number(num) => acc + num,
    })
}

fn extract_col<T>(sheet: &Vec<Vec<T>>, col_index: usize) -> Vec<&T> {
    sheet
        .iter()
        .map(|row| {
            row.get(col_index)
                .expect("Column index should not be out of bounds")
        })
        .collect()
}

fn parse_input(s: &str) -> Vec<Vec<Token>> {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|token| {
                    if let Ok(num) = token.parse::<u64>() {
                        Token::Number(num)
                    } else if token.len() == 1 {
                        Token::Operator(token.chars().next().unwrap())
                    } else {
                        panic!("Invalid token: {}", token);
                    }
                })
                .collect()
        })
        .collect()
}

fn vertical_parse(s: &str) -> (Vec<Token>, Vec<Vec<u64>>) {
    let mut rows: Vec<&str> = s.lines().collect();
    let ops = get_operations(&mut rows);
    let (list_str_numbers, ranges) = get_list_of_numbers_and_slices(&mut rows);

    let mut number_groups: Vec<_> = Vec::new();

    for range in ranges {
        let slice = &list_str_numbers[range];
        let tokens: Vec<u64> = slice.iter().map(|s| {
            s.trim().parse::<u64>().expect("should be parseable")
        })
        .collect();
        number_groups.push(tokens);
    }

    (ops, number_groups)
}

fn get_operations(rows: &mut Vec<&str>) -> Vec<Token> {
    rows
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|str| {
            match str
                .chars()
                .next()
                .expect("should be single character operators")
            {
                '*' => Token::Operator('*'),
                '+' => Token::Operator('+'),
                _ => panic!("expected only operators"),
            }
        })
        .collect()

}

fn get_list_of_numbers_and_slices(rows: &mut Vec<&str>) -> (Vec<String>, Vec<Range<usize>>)  {
    let rows: Vec<&[u8]> = rows.into_iter().map(|&mut str| str.as_bytes()).collect();
    let width = rows[0].len();

    let mut ranges: Vec<Range<usize>> = Vec::new();
    let mut start = None;

    // let matrix_by_col = vec![vec![]];
    let mut list_str_numbers = Vec::new();

    for col in 0..width {
        let str_numbers: String = rows.iter().map(|row| row[col] as char).collect();
        list_str_numbers.push(str_numbers);

        let occupied = rows.iter().any(|row| {
            row.get(col).is_some_and(|b| !b.is_ascii_whitespace())
        });

        match (start, occupied) {
            (None, true) => start = Some(col),
            (Some(s), false) => {
                ranges.push(s..col);
                start = None;
            },
            _ => {}
        }
    }

    if let Some(s) = start {
        ranges.push(s..width);
    }

    (list_str_numbers, ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn puzzle_one_example_input() {
        assert_eq!(process_puzzle_one(EXAMPLE_INPUT), 4277556);
    }

    #[test]
    fn puzzle_two_example_input() {
        assert_eq!(process_puzzle_two(EXAMPLE_INPUT), 3263827);
    }

    #[test]
    fn test_vertical_parse() {
        let one_operation = r#"123
 45
  6
*
"#;
        let operator_result = vec![Token::Operator('*')];
        let num_result = vec![vec![
            1,
            24,
            356,
        ]];
        assert_eq!((operator_result, num_result), vertical_parse(one_operation));
    }
}
