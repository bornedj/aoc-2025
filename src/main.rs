use std::{error::Error, fs, path::Path};

#[derive(Debug)]
enum Direction {
    Left(i16),
    Right(i16)
}

fn direction(line: &str) -> Direction {
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

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let file = fs::read_to_string(Path::new("/home/daniel/github/advent-of-code-2025/src/assets/day-one-puzzle-one-input.txt"))?;
    let directions: Vec<Direction> = file.lines().map(direction).collect();

    let mut position = 50;
    let mut count = 0;

    directions.iter().for_each(|direction| {
        match direction {
            Direction::Left(num) => position = (position - num).rem_euclid(100),
            Direction::Right(num) => position = (position + num).rem_euclid(100)
        };
        if position == 0 {
            count += 1;
        }
    });

    println!("{count}");
    Ok(())
}
