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

const DIAL_RANGE: i16 = 100;
fn spin(pos: i16, rot: i16) -> (i16, i16) {
    let dial_long = pos + rot;
    let mut revs = (dial_long / DIAL_RANGE).abs();
    let new_dial = dial_long.rem_euclid(DIAL_RANGE);

    if pos != 0 && dial_long <=0 {
        revs += 1;
    }
    (new_dial, revs)
}

pub fn process_puzzle_two(directions: Vec<Direction>) -> u16 {
    let mut curr_pos = 50;
    let mut curr_count = 0;

    for direction in directions {
        match direction {
            Direction::Right(rot) => {
                let (pos, count) = spin(curr_pos, rot);
                curr_pos = pos;
                curr_count += count;
            },
            Direction::Left(rot) => {
                let (pos, count) = spin(curr_pos, -rot);
                curr_pos = pos;
                curr_count += count;
            }
        }
    }
    curr_count as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_for_puzzle_two_returns_6() {
        let input = vec!("L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82");
        let directions = input.into_iter().map(direction).collect();

        assert_eq!(6, process_puzzle_two(directions));
    }

    #[test]
    fn test_spin() {
        assert_eq!((90, 3), spin(50, -260));
    }
}
