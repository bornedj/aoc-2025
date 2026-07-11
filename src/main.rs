use std::{error::Error, fs, path::Path};
use advent_of_code_2025::Direction;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let file = fs::read_to_string(Path::new("/home/daniel/github/advent-of-code-2025/src/assets/day-one-puzzle-one-input.txt"))?;
    let directions: Vec<Direction> = file.lines().map(advent_of_code_2025::direction).collect();

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
