use std::{error::Error, fs, path::Path};
use advent_of_code_2025::{Direction, process_puzzle_one, process_puzzle_two};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let file = fs::read_to_string(Path::new("/home/daniel/github/advent-of-code-2025/src/assets/day-one-puzzle-one-input.txt"))?;
    let directions: Vec<Direction> = file.lines().map(advent_of_code_2025::direction).collect();

    let position = 50;
    let mut count = 0;

    count = process_puzzle_one(directions.clone(), position, count);
    println!("puzzle one {count}");

    let position = 50;
    let mut count = 0;
    count = process_puzzle_two(directions, position, count);
    println!("puzzle two {count}");

    Ok(())
}
