#[derive(Debug)]
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

