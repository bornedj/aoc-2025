#[derive(Debug, Clone)]
pub enum Direction {
    Left(i16),
    Right(i16)
}

pub fn direction(line: &str) -> Direction {
    if let Some(distance) = line.strip_prefix('L') {
        let distance = distance.parse::<i16>().expect("must be a parseable number input");
        return Direction::Left(distance)
    } else if let Some(distance) = line.strip_prefix('R') {
        let distance = distance.parse::<i16>().expect("must be a parseable number input");
        return Direction::Right(distance)
    } else {
        panic!("unexpected prefix")
    }
}

pub fn process_puzzle_one(directions: Vec<Direction>, mut pos: i16, mut count: u16) -> u16 {
    directions.iter().for_each(|direction| {
        match direction {
            Direction::Left(num) => pos = (pos - num).rem_euclid(100),
            Direction::Right(num) => pos = (pos + num).rem_euclid(100)
        };
        if pos == 0 {
            count += 1;
        }
    });
    count
}

pub fn process_puzzle_two(directions: Vec<Direction>, mut pos: i16, mut count: u16) -> u16 {
    16
}

#[cfg(test)]
mod tests {
    use crate::process_puzzle_two;

use super::direction;

    #[test]
    fn test_input_for_puzzle_two_returns_6() {
        let input = vec!("L68", "L30", "R48", "L5", "R60", "L55", "L99", "R14", "L82");
        let pos = 50;
        let count = 0;
        let directions = input.into_iter().map(direction).collect();

        assert_eq!(6, process_puzzle_two(directions, pos, count));
    }
}
