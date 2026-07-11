use std::{error::Error, fs, path::Path};
use advent_of_code_2025::{day_one::{Direction, process_puzzle_one, process_puzzle_two}, day_two::process_day_two};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // day one
    {
        let file = fs::read_to_string(Path::new("/home/daniel/github/advent-of-code-2025/src/assets/day-one-puzzle-one-input.txt"))?;
        let directions: Vec<Direction> = file.lines().map(advent_of_code_2025::day_one::direction).collect();

        let mut count;

        count = process_puzzle_one(directions.clone());
        println!("day one - puzzle one {count}");

        count = process_puzzle_two(directions);
        println!("day one - puzzle two {count}");
    }

    // day two
    {
        let file = fs::read_to_string(Path::new("./src/assets/day-two-input.txt"))?;
        let result = process_day_two(&file.trim());
        println!("day two {result}");
    }

    Ok(())
}
