use std::thread;
use std::{collections::HashSet, ops::RangeInclusive};
use std::sync::mpsc;

pub fn puzzle_one(input: &str) -> usize {
    let (ranges, ingredients) = parse_input(input);
    ingredients
        .iter()
        .filter(|ingredient| {
            ranges.iter().any(|range| range.contains(ingredient))
        })
        .count()
}

pub fn puzzle_two(input: &str) -> u64 {
    let mut ranges = threaded_parse(input);
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut merged_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    for range in ranges {
        match merged_ranges.last_mut() {
            Some(last_range) if *range.start() <= last_range.end().saturating_add(1) => {
                let start = *last_range.start();
                let end = (*last_range.end()).max(*range.end());
                *last_range = start..=end;
            },
            _ => merged_ranges.push(range), // first or non-overlapping range
        }
    }

    merged_ranges.iter().map(|range| range.end() - range.start() + 1).sum()
}

fn threaded_parse(input: &str) -> Vec<RangeInclusive<u64>> {
    let (tx, rx) = mpsc::channel();
    thread::scope(|s| {
        for line in input.lines() {
            let tx_clone = tx.clone();
            s.spawn(move || {
                if line.contains('-') {
                    let range = transform_line_to_range(line);
                    println!("sending range {:?}", range);
                    tx_clone.send(range).expect("Failed to send vec range");
                }
            });
        }
    });

    drop(tx);
    rx.iter().collect()
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    input.lines().map(&str::trim).for_each(|line| {
        if line.contains('-') {
            let range = transform_line_to_range(line);
            ranges.push(range);
        } else if line.len() > 0 {
            let ingredient = line.parse::<u64>().expect("expected number");
            ingredients.push(ingredient);
        }
    });
    (ranges, ingredients)
}

fn transform_line_to_range(line: &str) -> RangeInclusive<u64> {
    let mut parts = line.split('-');
    let start = parts
        .next()
        .expect("expected start of range")
        .parse::<u64>()
        .expect("expected number");
    let end = parts
        .next()
        .expect("expected end of range")
        .parse::<u64>()
        .expect("expected number");
    start..=end
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn puzzle_one_example_input() {
        assert_eq!(puzzle_one(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn puzzle_two_example_input() {
        assert_eq!(puzzle_two(EXAMPLE_INPUT), 14);
    }
}
