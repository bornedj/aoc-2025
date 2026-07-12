#[derive(Debug)]
enum Token {
    Number(u64),
    Operator(char),
}

pub fn process_puzzle_one(input: &str) -> u64 {
    let mut home_work_sheet = parse_input(input);
    let operators = home_work_sheet
        .pop()
        .expect("Expected operators row at the end");

    let mut total = 0;

    for (i, operator) in operators.iter().enumerate() {
        match operator {
            Token::Number(_) => panic!("Expected operator, found number"),
            Token::Operator(op) => {
                let col = extract_col(&home_work_sheet, i);
                match op {
                    '*' => total += multiply_col(&col),
                    '+' => total += add_col(&col),
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

fn extract_col(sheet: &Vec<Vec<Token>>, col_index: usize) -> Vec<&Token> {
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
}
